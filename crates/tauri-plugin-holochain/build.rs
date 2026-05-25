const COMMANDS: &[&str] = &["sign_zome_call"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
