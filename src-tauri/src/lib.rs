use serde::Serialize;
use std::{
    path::Path,
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
    time::Duration,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppStatus {
    application: &'static str,
    phase: &'static str,
    camera_access: bool,
}

#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
enum CheckStatus {
    Ready,
    Attention,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PreflightCheck {
    id: &'static str,
    label: &'static str,
    status: CheckStatus,
    summary: String,
    next_step: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PreflightResult {
    ready: bool,
    summary: String,
    checks: Vec<PreflightCheck>,
}

#[derive(Default)]
struct PipelineManager {
    child: Mutex<Option<Child>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PipelineConfiguration {
    source_name: &'static str,
    source_available: bool,
    virtual_camera_available: bool,
    transform: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PipelineStatus {
    state: &'static str,
    message: String,
}

#[tauri::command]
fn get_app_status() -> AppStatus {
    AppStatus {
        application: "Linux Camera Controller",
        phase: "Foundation ready",
        camera_access: false,
    }
}

fn command_succeeds(program: &str, arguments: &[&str]) -> bool {
    Command::new(program)
        .args(arguments)
        .output()
        .is_ok_and(|output| output.status.success())
}

fn command_output(program: &str, arguments: &[&str]) -> Option<String> {
    Command::new(program)
        .args(arguments)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
}

fn find_loopback_camera(device_list: &str) -> Option<String> {
    let mut is_loopback_device = false;

    for line in device_list.lines() {
        if line.starts_with(char::is_whitespace) {
            let device = line.trim();
            if is_loopback_device && device.starts_with("/dev/video") {
                return Some(device.to_owned());
            }
        } else {
            is_loopback_device = line.contains("v4l2loopback");
        }
    }

    None
}

fn find_streamcam_camera(device_list: &str) -> Option<String> {
    let mut is_streamcam = false;

    for line in device_list.lines() {
        if line.starts_with(char::is_whitespace) {
            let device = line.trim();
            if is_streamcam && device.starts_with("/dev/video") {
                return Some(device.to_owned());
            }
        } else {
            is_streamcam = line.contains("Logitech StreamCam");
        }
    }

    None
}

fn list_video_devices() -> Result<String, String> {
    command_output("v4l2-ctl", &["--list-devices"]).ok_or_else(|| {
        "Camera discovery is unavailable. Check that V4L2 tools are installed.".to_owned()
    })
}

fn resolved_pipeline_devices() -> Result<(String, String), String> {
    let devices = list_video_devices()?;
    let source = find_streamcam_camera(&devices).ok_or_else(|| {
        "Logitech StreamCam was not found. Connect it, then refresh readiness.".to_owned()
    })?;
    let virtual_camera = find_loopback_camera(&devices).ok_or_else(|| {
        "No configured virtual camera was found. Complete the first-time setup, then refresh readiness."
            .to_owned()
    })?;

    Ok((source, virtual_camera))
}

fn pipeline_status(manager: &PipelineManager) -> Result<PipelineStatus, String> {
    let mut child = manager
        .child
        .lock()
        .map_err(|_| "The camera pipeline state is unavailable. Restart the app.".to_owned())?;

    let is_running = match child.as_mut() {
        Some(process) => process
            .try_wait()
            .map_err(|_| "Could not read the camera pipeline state. Restart the app.".to_owned())?
            .is_none(),
        None => false,
    };

    if !is_running {
        *child = None;
    }

    Ok(PipelineStatus {
        state: if is_running { "running" } else { "stopped" },
        message: if is_running {
            "The rotated StreamCam pipeline is running.".to_owned()
        } else {
            "The camera pipeline is stopped.".to_owned()
        },
    })
}

#[tauri::command]
fn get_pipeline_configuration() -> PipelineConfiguration {
    let devices = list_video_devices().unwrap_or_default();

    PipelineConfiguration {
        source_name: "Logitech StreamCam",
        source_available: find_streamcam_camera(&devices).is_some(),
        virtual_camera_available: find_loopback_camera(&devices).is_some(),
        transform: "180° rotation",
    }
}

#[tauri::command]
fn get_pipeline_status(
    manager: tauri::State<'_, PipelineManager>,
) -> Result<PipelineStatus, String> {
    pipeline_status(&manager)
}

#[tauri::command]
fn start_pipeline(manager: tauri::State<'_, PipelineManager>) -> Result<PipelineStatus, String> {
    if pipeline_status(&manager)?.state == "running" {
        return Err("The camera pipeline is already running.".to_owned());
    }

    if !command_succeeds("ffmpeg", &["-version"]) {
        return Err(
            "FFmpeg is not available. Resolve the readiness check before starting.".to_owned(),
        );
    }

    let (source, virtual_camera) = resolved_pipeline_devices()?;
    let mut process = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "warning",
            "-nostdin",
            "-f",
            "v4l2",
            "-video_size",
            "1280x720",
            "-framerate",
            "30",
            "-i",
            &source,
            "-vf",
            "hflip,vflip,format=yuyv422",
            "-pix_fmt",
            "yuyv422",
            "-f",
            "v4l2",
            &virtual_camera,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| "FFmpeg could not start. Check camera readiness and try again.".to_owned())?;

    thread::sleep(Duration::from_millis(400));
    if process
        .try_wait()
        .map_err(|_| "Could not verify the FFmpeg process. Try again.".to_owned())?
        .is_some()
    {
        return Err(
            "FFmpeg stopped immediately. The camera may be in use or the selected mode is unavailable."
                .to_owned(),
        );
    }

    let mut child = manager
        .child
        .lock()
        .map_err(|_| "The camera pipeline state is unavailable. Restart the app.".to_owned())?;
    *child = Some(process);

    Ok(PipelineStatus {
        state: "running",
        message: "The rotated StreamCam pipeline is running. Select StreamCam Rotated in Teams."
            .to_owned(),
    })
}

#[tauri::command]
fn stop_pipeline(manager: tauri::State<'_, PipelineManager>) -> Result<PipelineStatus, String> {
    let mut child = manager
        .child
        .lock()
        .map_err(|_| "The camera pipeline state is unavailable. Restart the app.".to_owned())?;

    let Some(mut process) = child.take() else {
        return Ok(PipelineStatus {
            state: "stopped",
            message: "The camera pipeline is already stopped.".to_owned(),
        });
    };

    if process
        .try_wait()
        .map_err(|_| "Could not read the camera pipeline state. Restart the app.".to_owned())?
        .is_none()
    {
        process
            .kill()
            .map_err(|_| "The camera pipeline could not be stopped. Restart the app.".to_owned())?;
    }

    let _ = process.wait();

    Ok(PipelineStatus {
        state: "stopped",
        message: "The camera pipeline stopped and released the StreamCam.".to_owned(),
    })
}

#[tauri::command]
fn get_preflight() -> PreflightResult {
    let ffmpeg_available = command_succeeds("ffmpeg", &["-version"]);
    let v4l2_available = command_succeeds("v4l2-ctl", &["--version"]);
    let loopback_installed = command_succeeds("modinfo", &["v4l2loopback"]);
    let loopback_loaded = Path::new("/sys/module/v4l2loopback").exists();
    let virtual_camera = if v4l2_available {
        command_output("v4l2-ctl", &["--list-devices"])
            .as_deref()
            .and_then(find_loopback_camera)
    } else {
        None
    };

    let mut checks = vec![
        PreflightCheck {
            id: "ffmpeg",
            label: "FFmpeg",
            status: if ffmpeg_available {
                CheckStatus::Ready
            } else {
                CheckStatus::Attention
            },
            summary: if ffmpeg_available {
                "Available for the camera pipeline.".to_owned()
            } else {
                "FFmpeg was not found.".to_owned()
            },
            next_step: (!ffmpeg_available).then(|| {
                "Install an FFmpeg build for your distro, then refresh this check.".to_owned()
            }),
        },
        PreflightCheck {
            id: "v4l2-tools",
            label: "V4L2 tools",
            status: if v4l2_available {
                CheckStatus::Ready
            } else {
                CheckStatus::Attention
            },
            summary: if v4l2_available {
                "Camera discovery tools are available.".to_owned()
            } else {
                "The v4l2-ctl tool was not found.".to_owned()
            },
            next_step: (!v4l2_available)
                .then(|| "Install v4l-utils, then refresh this check.".to_owned()),
        },
    ];

    let virtual_camera_check = if !loopback_installed {
        PreflightCheck {
            id: "virtual-camera",
            label: "Virtual camera",
            status: CheckStatus::Attention,
            summary: "The v4l2loopback kernel module is not installed.".to_owned(),
            next_step: Some(
                "Install v4l2loopback and complete the first-time setup guide.".to_owned(),
            ),
        }
    } else if !loopback_loaded {
        PreflightCheck {
            id: "virtual-camera",
            label: "Virtual camera",
            status: CheckStatus::Attention,
            summary: "v4l2loopback is installed but not loaded.".to_owned(),
            next_step: Some(
                "Restart after completing the first-time setup guide, then refresh this check."
                    .to_owned(),
            ),
        }
    } else if let Some(device) = virtual_camera {
        PreflightCheck {
            id: "virtual-camera",
            label: "Virtual camera",
            status: CheckStatus::Ready,
            summary: format!("A v4l2loopback camera is ready at {device}."),
            next_step: None,
        }
    } else {
        PreflightCheck {
            id: "virtual-camera",
            label: "Virtual camera",
            status: CheckStatus::Attention,
            summary: "v4l2loopback is loaded, but no virtual camera was found.".to_owned(),
            next_step: Some(
                "Review the first-time setup guide and restart before refreshing this check."
                    .to_owned(),
            ),
        }
    };

    checks.push(virtual_camera_check);
    let ready = checks
        .iter()
        .all(|check| check.status == CheckStatus::Ready);

    PreflightResult {
        ready,
        summary: if ready {
            "Your system is ready for the camera pipeline.".to_owned()
        } else {
            "Setup is needed before a camera pipeline can start.".to_owned()
        },
        checks,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(PipelineManager::default())
        .invoke_handler(tauri::generate_handler![
            get_app_status,
            get_preflight,
            get_pipeline_configuration,
            get_pipeline_status,
            start_pipeline,
            stop_pipeline
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::{find_loopback_camera, find_streamcam_camera};

    #[test]
    fn finds_a_loopback_video_device() {
        let devices = "StreamCam Rotated (platform:v4l2loopback-010):\n\t/dev/video10\n\nLogitech StreamCam:\n\t/dev/video1\n";

        assert_eq!(
            find_loopback_camera(devices),
            Some("/dev/video10".to_owned())
        );
    }

    #[test]
    fn ignores_physical_camera_devices() {
        let devices = "Logitech StreamCam:\n\t/dev/video1\n\t/dev/video2\n";

        assert_eq!(find_loopback_camera(devices), None);
    }

    #[test]
    fn finds_the_first_streamcam_video_device() {
        let devices = "Logitech StreamCam:\n\t/dev/video1\n\t/dev/video2\n";

        assert_eq!(
            find_streamcam_camera(devices),
            Some("/dev/video1".to_owned())
        );
    }
}
