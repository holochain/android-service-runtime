const COMMANDS: &[&str] = &[
    "start",
    "stop",
    "is_ready",
    "install_app",
    "uninstall_app",
    "enable_app",
    "disable_app",
    "list_apps",
    "is_app_installed",
    "ensure_app_websocket",
    "sign_zome_call",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
