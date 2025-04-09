#![cfg(mobile)]

mod error;
mod mobile;
mod types;

pub use error::{Error, Result};
pub use holochain_conductor_runtime_types_ffi::AppAuthFfi;
use mobile::HolochainServiceConsumer;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
pub use types::SetupAppConfig;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`]
pub trait HolochainServiceConsumerExt<R: Runtime> {
    fn holochain_service_consumer(&self) -> &HolochainServiceConsumer<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainServiceConsumerExt<R> for T {
    fn holochain_service_consumer(&self) -> &HolochainServiceConsumer<R> {
        self.state::<HolochainServiceConsumer<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("holochain-service-consumer")
        .setup(|app, api| {
            let plugin = mobile::init(app, api)?;
            app.manage(plugin);
            Ok(())
        })
        .build()
}
