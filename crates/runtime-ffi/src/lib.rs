hc_uniffi::setup_scaffolding!();

extern crate android_logger;
extern crate log;

mod autostart;
mod error;
mod multi_thread;
mod runtime;

pub use autostart::*;
pub use runtime::*;
