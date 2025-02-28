use holochain_conductor_runtime::RuntimeError;
use holochain_types::app::AppBundleError;
use url2::Url2Error;

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum RuntimeErrorFfi {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),

    #[error(transparent)]
    Config(#[from] RuntimeConfigErrorFfi),

    #[error(transparent)]
    DecodeAppBundle(#[from] AppBundleError)
}

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum RuntimeConfigErrorFfi {
    #[error(transparent)]
    Url2Error(#[from] Url2Error),
}

pub type RuntimeResultFfi<T> = Result<T, RuntimeErrorFfi>;
