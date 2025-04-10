#![cfg(mobile)]

use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::sync::LazyLock;

static LOGGER: LazyLock<SimpleLogger> = LazyLock::new(|| SimpleLogger::new());

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Ignore error that logger is already initialized
    let _ = log::set_logger(&*LOGGER);
    log::set_max_level(LevelFilter::Warn);

    tauri::Builder::default()
        .plugin(tauri_plugin_holochain_service::init())
        .plugin(tauri_plugin_log::Builder::new().skip_logger().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
