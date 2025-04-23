#![cfg(mobile)]

mod error;
mod mobile;

pub use error::{Error, Result};
use mobile::HolochainService;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain-service APIs.
pub trait HolochainServiceExt<R: Runtime> {
    fn holochain_service(&self) -> &HolochainService<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainServiceExt<R> for T {
    fn holochain_service(&self) -> &HolochainService<R> {
        self.state::<HolochainService<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("holochain-service")
        .setup(|app, api| {
            let dialog = mobile::init(app, api)?;
            app.manage(dialog);
            Ok(())
        })
        .build()
}
