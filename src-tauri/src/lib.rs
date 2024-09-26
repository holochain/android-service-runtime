const APP_ID: &str = "tauri-app";
const HAPP_PATH: &str = "../../workdir/tauri-app.happ";
const SIGNAL_URL: &str = "wss://signal.holo.host";
const BOOTSTRAP_URL: &str = "https://bootstrap.holo.host";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_holochain_foreground_service::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
