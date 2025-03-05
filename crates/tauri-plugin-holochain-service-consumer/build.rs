const COMMANDS: &[&str] = &[
    "install_app",
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
