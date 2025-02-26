uniffi::setup_scaffolding!();

#[macro_use] extern crate log;
extern crate android_logger;

mod runtime;
pub use runtime::*;