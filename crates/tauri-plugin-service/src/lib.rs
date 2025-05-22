//! A Tauri plugin for holochain runtime apps.
//!
//! With this plugin, you can write an android app that bundles a holochain conductor,
//! runs it within a [foreground service](https://developer.android.com/develop/background-work/services/fgs/launch).
//! 
//! Apps can make use of this holochain conductor with the client-side plugin  [tauri-plugin-holochain-service-client](../tauri-plugin-client/README.md).
//!
//! ## Usage
//!
//! See the [example-client-app](https://github.com/holochain/android-service-runtime/blob/main/apps/example-client-app/src-tauri/src/lib.rs) 
//! for a demo of how to integrate this plugin in your Tauri app.


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
