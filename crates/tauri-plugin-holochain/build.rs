const COMMANDS: &[&str] = &["sign_zome_call", "sign_payload", "app_request"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
