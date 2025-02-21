#![cfg(mobile)]

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

mod mobile;
mod error;
mod types;

pub use error::{Error, Result};
pub use types::*;

use mobile::HolochainService;

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