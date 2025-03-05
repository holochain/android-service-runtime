uniffi::setup_scaffolding!();

extern crate log;
extern crate android_logger;

mod config;
mod error;
mod runtime;
mod types;
pub use runtime::*;
