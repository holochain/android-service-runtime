use holochain::{
    conductor::{api::AdminResponse, error::ConductorError, interface::error::InterfaceError},
    prelude::CellId,
};
use kitsune_p2p_types::dependencies::lair_keystore_api::dependencies::one_err::OneErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    Conductor(#[from] ConductorError),

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

    #[error("Lair Error")]
    Lair(OneErr),
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
