// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  #[cfg(all(mobile, target_os="android"))]
  tauri_app_lib::run();

  #[cfg(not(all(mobile, target_os="android")))]
  unimplemented!("Only mobile android is supported.");
}
