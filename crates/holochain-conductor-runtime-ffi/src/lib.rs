uniffi::setup_scaffolding!();

extern crate android_logger;
extern crate log;

mod error;
mod runtime;

pub use runtime::*;
