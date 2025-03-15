uniffi::setup_scaffolding!();

extern crate log;
extern crate android_logger;

mod runtime;
mod error;

pub use runtime::*;
