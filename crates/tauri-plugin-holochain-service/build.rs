const COMMANDS: &[&str] = &[
    "launch",
    "shutdown",
    "install_app",
    "uninstall_app",
    "enable_app",
    "disable_app",
    "list_installed_apps",
    "is_app_installed",
    "ensure_app_websocket",
    "sign_zome_call",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
