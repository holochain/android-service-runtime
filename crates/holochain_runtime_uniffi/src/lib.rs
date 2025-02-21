uniffi::setup_scaffolding!();

#[macro_use] extern crate log;
extern crate android_logger;

mod error;
mod runtime;
mod types;
mod config;
pub use runtime::*;