#![cfg(mobile)]

mod error;
mod mobile;

pub use error::{Error, Result};
pub use holochain_conductor_runtime_types_ffi::RuntimeNetworkConfigFfi;
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

/// Initialize the plugin, passing in the initial network config to apply to the runtime.
///
/// Note that peers MUST have the same `bootstrap_url` and `signal_url` to communicate
/// with each other.
pub fn init<R: Runtime>(config: RuntimeNetworkConfigFfi) -> TauriPlugin<R> {
    Builder::new("holochain-service")
        .setup(|app, api| {
            let dialog = mobile::init(app, api, config)?;
            app.manage(dialog);
            Ok(())
        })
        .build()
}
