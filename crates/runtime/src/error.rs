use holochain::{
    conductor::{api::AdminResponse, error::ConductorError, interface::error::InterfaceError},
    prelude::{AppBundleError, CellId},
};
use lair_keystore_api::dependencies::one_err::OneErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    Conductor(#[from] ConductorError),

    #[error("Conductor was never started")]
    ConductorNotStarted,

    #[error("Failed to shutdown conductor {0}")]
    ConductorShutdown(String),

    #[error("Admin Api Interface Error: {0}")]
    AdminApiInterface(#[from] InterfaceError),

    #[error("Admin Api App Enabled Errors: {0:?}")]
    AdminApiAppEnabled(Vec<(CellId, String)>),

    #[error("Admin Api Bad Response: {0:?}")]
    AdminApiBadResponse(AdminResponse),

    #[error("Move to Locked Memory Error")]
    MoveToLockedMem(OneErr),

    #[error("Failed to sign zome call {0}")]
    ZomeCallParamsInvalid(String),

    #[error("Lair Error")]
    Lair(OneErr),

    #[error("App Bundle Error")]
    AppBundle(#[from] AppBundleError),

    #[error("InstalledAppId must be specified when installing an app")]
    InstalledAppIdNotSpecified,

    #[error("Failed to read persisted data from file: {0}")]
    PersistedFileReadError(String),

    #[error("Failed to write persisted data to file: {0}")]
    PersistedFileWriteError(String),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
