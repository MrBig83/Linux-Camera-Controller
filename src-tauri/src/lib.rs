use serde::Serialize;
use std::{path::Path, process::Command};

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
        .invoke_handler(tauri::generate_handler![get_app_status, get_preflight])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::find_loopback_camera;

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
}
