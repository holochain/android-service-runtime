#![cfg(mobile)]

mod error;
mod mobile;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};
pub use error::{Error, Result};
use mobile::HolochainServiceConsumer;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain-service APIs.
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
            let dialog = mobile::init(app, api)?;
            app.manage(dialog);
            Ok(())
        })
        .build()
}
