const COMMANDS: &[&str] = &[
  "launch", "shutdown", "get_admin_port", "install_app", "uninstall_app", "enable_app", "disable_app", 
  "list_installed_apps", "is_app_installed", "app_websocket_auth", "sign_zome_call",
];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}