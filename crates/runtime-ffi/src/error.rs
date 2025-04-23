use holochain_conductor_runtime::RuntimeError;
use holochain_types::app::AppBundleError;
use url2::Url2Error;

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum RuntimeErrorFfi {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),

    #[error("Runtime is not started")]
    RuntimeNotStarted,

    #[error(transparent)]
    Config(#[from] Url2Error),

    #[error(transparent)]
    DecodeAppBundle(#[from] AppBundleError),
}

pub type RuntimeResultFfi<T> = Result<T, RuntimeErrorFfi>;
