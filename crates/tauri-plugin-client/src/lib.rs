//! A Tauri plugin for holochain end-user apps.
//!
//! With this plugin, you can write an android app that relies on holochain,
//! without bundling the holochain conductor.
//! 
//! Instead, this plugin expects that you have *another* android app installed,
//! with the tauri plugin `tauri-plugin-holochain-service`,
//! and will use that app's holochain conductor.
//!
//! ## Usage
//!
//! See the [example-client-app](https://github.com/holochain/android-service-runtime/blob/main/apps/example-client-app/src-tauri/src/lib.rs) 
//! for a demo of how to integrate this plugin in your Tauri app.

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
    Builder::new("holochain-service-client")
        .setup(|app, api| {
            let plugin = mobile::init(app, api)?;
            app.manage(plugin);
            Ok(())
        })
        .build()
}
