#![cfg(mobile)]

use std::sync::LazyLock;
use std::sync::Once;
use simple_logger::SimpleLogger;
use log::LevelFilter;

// Prevent the logger from being initialized twice
// when the app is closed and re-launched
static LOGGER: LazyLock<SimpleLogger> = LazyLock::new(|| {
    SimpleLogger::new()
});
static INIT_LOGGER: Once = Once::new();
fn init_logger() {
    INIT_LOGGER.call_once(|| {
        log::set_logger(&*LOGGER).unwrap();
        log::set_max_level(LevelFilter::Warn);
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logger();

    tauri::Builder::default()
        .plugin(tauri_plugin_holochain_service::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .skip_logger()
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
