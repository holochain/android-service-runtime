use tauri_plugin_holochain_service_consumer::{HolochainServiceConsumerExt, SetupAppConfig};
use uuid::Uuid;
use std::collections::HashMap;

pub const APP_ID: &'static str = "forum";
pub const HAPP_BUNDLE_BYTES: &'static [u8] = include_bytes!("../../workdir/forum.happ");

#[allow(unused_mut)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_holochain_service_consumer::init())
        .setup(|app| {
            app.handle()
                .holochain_service_consumer()
                .setup_app_main_window(
                    SetupAppConfig {
                        app_id: APP_ID.into(),
                        happ_bundle_bytes: HAPP_BUNDLE_BYTES.into(),
                        network_seed: Uuid::new_v4().to_string(),
                        roles_settings: HashMap::new(),
                        enable_after_install: true
                    }
                )?
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
