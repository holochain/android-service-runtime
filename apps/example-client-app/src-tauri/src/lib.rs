use tauri_plugin_holochain_service_consumer::HolochainServiceConsumerExt;
use uuid::Uuid;

pub const APP_ID: &'static str = "forum";
pub const HAPP_BUNDLE_BYTES: &'static [u8] = include_bytes!("../../workdir/forum.happ");

#[allow(unused_mut)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Warn)
                .build(),
        )
        .plugin(tauri_plugin_holochain_service_consumer::init())
        .setup(|app| {
            app.handle()
                .holochain_service_consumer()
                .main_window_builder(
                    APP_ID.into(),
                    HAPP_BUNDLE_BYTES.into(),
                    Uuid::new_v4().to_string(),
                    true
                )
                .expect("Failed to build window")
                .build()
                .expect("Failed to open main window");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
