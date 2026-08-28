use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AppStatus {
    application: &'static str,
    phase: &'static str,
    camera_access: bool,
}

#[tauri::command]
fn get_app_status() -> AppStatus {
    AppStatus {
        application: "Linux Camera Controller",
        phase: "Foundation ready",
        camera_access: false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_app_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
