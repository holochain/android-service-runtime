const COMMANDS: &[&str] = &["sign_zome_call", "app_request"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
