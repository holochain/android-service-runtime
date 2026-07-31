#![cfg(mobile)]

use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::sync::LazyLock;
use tauri_plugin_holochain_service::RuntimeNetworkConfigFfi;

static LOGGER: LazyLock<SimpleLogger> = LazyLock::new(|| SimpleLogger::new());

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Ignore error that logger is already initialized
    let _ = log::set_logger(&*LOGGER);
    log::set_max_level(LevelFilter::Warn);

    tauri::Builder::default()
        .plugin(tauri_plugin_holochain_service::init(
            RuntimeNetworkConfigFfi {
                bootstrap_url: "https://dev-test-bootstrap2.holochain.org".to_string(),
                signal_url: "wss://dev-test-bootstrap2.holochain.org".to_string(),
                relay_url: "https://use1-1.relay.n0.iroh-canary.iroh.link./".to_string(),
                ice_urls: vec!["stun:stun.nextcloud.com:443".to_string()],
            },
        ))
        .plugin(tauri_plugin_log::Builder::new().skip_logger().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
