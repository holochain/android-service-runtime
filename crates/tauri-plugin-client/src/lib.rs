#![cfg(mobile)]

mod error;
mod mobile;
mod types;

pub use error::{Error, Result};
pub use holochain_conductor_runtime_types_ffi::AppAuthFfi;
use mobile::HolochainServiceClient;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
pub use types::SetupAppConfig;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`]
pub trait HolochainServiceClientExt<R: Runtime> {
    fn holochain_service_client(&self) -> &HolochainServiceClient<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainServiceClientExt<R> for T {
    fn holochain_service_client(&self) -> &HolochainServiceClient<R> {
        self.state::<HolochainServiceClient<R>>().inner()
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
