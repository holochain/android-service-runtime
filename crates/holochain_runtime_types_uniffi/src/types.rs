use std::{collections::HashMap, time::Duration};

use holochain_conductor_api::{
    AppInfo, AppInfoStatus, CellInfo, ProvisionedCell, StemCell, ZomeCall,
};
use holochain_runtime::AppWebsocketAuth;
use holochain_types::{
    app::{DisabledAppReason, PausedAppReason, RoleSettings},
    dna::{
        hash_type::{Agent, Dna},
        HoloHash,
    },
    prelude::{
        CapSecret, CellId, ClonedCell, DnaModifiers, DnaModifiersOpt, ExternIO, FunctionName,
        Nonce256Bits, SerializedBytes, Timestamp, UnsafeBytes, YamlProperties, ZomeCallUnsigned,
        ZomeName,
    },
};

#[derive(uniffi::Record)]
pub struct DurationFFI {
    pub secs: u64,
    pub nanos: u32,
}

impl From<Duration> for DurationFFI {
    fn from(value: Duration) -> Self {
        Self {
            secs: value.as_secs(),
            nanos: value.subsec_nanos(),
        }
    }
}

impl From<DurationFFI> for Duration {
    fn from(val: DurationFFI) -> Self {
        Duration::from_nanos(val.secs * 1000 + val.nanos as u64)
    }
}

#[derive(uniffi::Record)]
pub struct DnaModifiersFFI {
    pub network_seed: String,
    pub properties: Vec<u8>,
    pub origin_time: i64,
    pub quantum_time: DurationFFI,
}

impl From<DnaModifiers> for DnaModifiersFFI {
    fn from(value: DnaModifiers) -> Self {
        Self {
            network_seed: value.network_seed,
            properties: value.properties.bytes().to_owned(),
            origin_time: value.origin_time.0,
            quantum_time: value.quantum_time.into(),
        }
    }
}

#[derive(uniffi::Record)]
pub struct DnaModifiersOptFFI {
    pub network_seed: Option<String>,
    pub properties: Option<Vec<u8>>,
    pub origin_time: Option<i64>,
    pub quantum_time: Option<DurationFFI>,
}

impl From<DnaModifiersOptFFI> for DnaModifiersOpt<YamlProperties> {
    fn from(val: DnaModifiersOptFFI) -> Self {
        DnaModifiersOpt {
            network_seed: val.network_seed,
            properties: val.properties.map(|p| {
                YamlProperties::try_from(SerializedBytes::from(UnsafeBytes::from(p))).unwrap()
            }),
            origin_time: val.origin_time.map(Timestamp),
            quantum_time: val.quantum_time.map(|d| d.into()),
        }
    }
}

#[derive(uniffi::Record)]
pub struct ProvisionedCellFFI {
    pub cell_id: CellIdFFI,
    pub dna_modifiers: DnaModifiersFFI,
    pub name: String,
}

impl From<ProvisionedCell> for ProvisionedCellFFI {
    fn from(value: ProvisionedCell) -> Self {
        Self {
            cell_id: value.cell_id.into(),
            dna_modifiers: value.dna_modifiers.into(),
            name: value.name,
        }
    }
}

#[derive(uniffi::Record)]
pub struct ClonedCellFFI {
    pub cell_id: CellIdFFI,
    pub clone_id: String,
    pub original_dna_hash: Vec<u8>,
    pub dna_modifiers: DnaModifiersFFI,
    pub name: String,
    pub enabled: bool,
}

impl From<ClonedCell> for ClonedCellFFI {
    fn from(value: ClonedCell) -> Self {
        Self {
            cell_id: value.cell_id.into(),
            clone_id: value.clone_id.0,
            original_dna_hash: value.original_dna_hash.get_raw_39().to_vec(),
            dna_modifiers: value.dna_modifiers.into(),
            name: value.name,
            enabled: value.enabled,
        }
    }
}

#[derive(uniffi::Record)]
pub struct StemCellFFI {
    pub original_dna_hash: Vec<u8>,
    pub dna_modifiers: DnaModifiersFFI,
    pub name: Option<String>,
}

impl From<StemCell> for StemCellFFI {
    fn from(value: StemCell) -> Self {
        Self {
            original_dna_hash: value.original_dna_hash.get_raw_39().to_vec(),
            dna_modifiers: value.dna_modifiers.into(),
            name: value.name,
        }
    }
}

#[derive(uniffi::Enum)]
pub enum CellInfoFFI {
    Provisioned(ProvisionedCellFFI),
    Cloned(ClonedCellFFI),
    Stem(StemCellFFI),
}

impl From<CellInfo> for CellInfoFFI {
    fn from(value: CellInfo) -> Self {
        match value {
            CellInfo::Provisioned(provisioned) => CellInfoFFI::Provisioned(provisioned.into()),
            CellInfo::Cloned(cloned) => CellInfoFFI::Cloned(cloned.into()),
            CellInfo::Stem(stem) => CellInfoFFI::Stem(stem.into()),
        }
    }
}

#[derive(uniffi::Enum)]
pub enum PausedAppReasonFFI {
    Error(String),
}

impl From<PausedAppReason> for PausedAppReasonFFI {
    fn from(value: PausedAppReason) -> Self {
        match value {
            PausedAppReason::Error(error) => PausedAppReasonFFI::Error(error),
        }
    }
}

#[derive(uniffi::Enum)]
pub enum DisabledAppReasonFFI {
    NeverStarted,
    NotStartedAfterProvidingMemproofs,
    DeletingAgentKey,
    User,
    Error(String),
}

impl From<DisabledAppReason> for DisabledAppReasonFFI {
    fn from(value: DisabledAppReason) -> Self {
        match value {
            DisabledAppReason::NeverStarted => DisabledAppReasonFFI::NeverStarted,
            DisabledAppReason::NotStartedAfterProvidingMemproofs => {
                DisabledAppReasonFFI::NotStartedAfterProvidingMemproofs
            }
            DisabledAppReason::DeletingAgentKey => DisabledAppReasonFFI::DeletingAgentKey,
            DisabledAppReason::User => DisabledAppReasonFFI::User,
            DisabledAppReason::Error(error) => DisabledAppReasonFFI::Error(error),
        }
    }
}

#[derive(uniffi::Enum)]
pub enum AppInfoStatusFFI {
    Paused { reason: PausedAppReasonFFI },
    Disabled { reason: DisabledAppReasonFFI },
    Running,
    AwaitingMemproofs,
}

impl From<AppInfoStatus> for AppInfoStatusFFI {
    fn from(value: AppInfoStatus) -> Self {
        match value {
            AppInfoStatus::Paused { reason: paused } => AppInfoStatusFFI::Paused {
                reason: paused.into(),
            },
            AppInfoStatus::Disabled { reason: disabled } => AppInfoStatusFFI::Disabled {
                reason: disabled.into(),
            },
            AppInfoStatus::Running => AppInfoStatusFFI::Running,
            AppInfoStatus::AwaitingMemproofs => AppInfoStatusFFI::AwaitingMemproofs,
        }
    }
}

#[derive(uniffi::Record)]
pub struct AppInfoFFI {
    /// The unique identifier for an installed app in this conductor
    pub installed_app_id: String,
    pub cell_info: HashMap<String, Vec<CellInfoFFI>>,
    pub status: AppInfoStatusFFI,
    pub agent_pub_key: Vec<u8>,
}

impl From<AppInfo> for AppInfoFFI {
    fn from(value: AppInfo) -> Self {
        let mut cell_info: HashMap<String, Vec<CellInfoFFI>> = HashMap::new();
        for entry in value.cell_info.into_iter() {
            let entry_cell_infos: Vec<CellInfoFFI> =
                entry.1.into_iter().map(|val| val.into()).collect();
            cell_info.insert(entry.0, entry_cell_infos);
        }

        Self {
            installed_app_id: value.installed_app_id,
            cell_info,
            status: value.status.into(),
            agent_pub_key: value.agent_pub_key.into_inner(),
        }
    }
}

#[derive(uniffi::Record)]
pub struct AppWebsocketAuthFFI {
    pub app_id: String,
    pub port: u16,
    pub token: Vec<u8>,
}

impl From<AppWebsocketAuth> for AppWebsocketAuthFFI {
    fn from(value: AppWebsocketAuth) -> Self {
        Self {
            app_id: value.app_id,
            port: value.app_websocket_port,
            token: value.token,
        }
    }
}

#[derive(uniffi::Record)]
pub struct CellIdFFI {
    pub dna_hash: Vec<u8>,
    pub agent_pub_key: Vec<u8>,
}

impl From<CellId> for CellIdFFI {
    fn from(value: CellId) -> Self {
        Self {
            dna_hash: value.dna_hash().get_raw_39().to_vec(),
            agent_pub_key: value.agent_pubkey().get_raw_39().to_vec(),
        }
    }
}

impl From<CellIdFFI> for CellId {
    fn from(val: CellIdFFI) -> Self {
        CellId::new(
            HoloHash::<Dna>::from_raw_39(val.dna_hash).unwrap(),
            HoloHash::<Agent>::from_raw_39(val.agent_pub_key).unwrap(),
        )
    }
}

#[derive(uniffi::Record)]
pub struct ZomeCallUnsignedFFI {
    pub provenance: Vec<u8>,
    pub cell_id: CellIdFFI,
    pub zome_name: String,
    pub fn_name: String,
    pub cap_secret: Option<Vec<u8>>,
    pub payload: Vec<u8>,
    pub nonce: Vec<u8>,
    pub expires_at: i64,
}

impl From<ZomeCallUnsigned> for ZomeCallUnsignedFFI {
    fn from(value: ZomeCallUnsigned) -> Self {
        Self {
            provenance: value.provenance.get_raw_39().to_vec(),
            cell_id: value.cell_id.into(),
            zome_name: value.zome_name.0.to_string(),
            fn_name: value.fn_name.into(),
            cap_secret: value.cap_secret.map(|s| s.as_ref().to_vec()),
            payload: value.payload.into(),
            nonce: value.nonce.into_inner().to_vec(),
            expires_at: value.expires_at.0,
        }
    }
}

impl From<ZomeCallUnsignedFFI> for ZomeCallUnsigned {
    fn from(val: ZomeCallUnsignedFFI) -> Self {
        let nonce: [u8; 32] = val.nonce.as_slice().try_into().unwrap();
        let cap_secret: Option<[u8; 64]> = val.cap_secret.map(|s| s.as_slice().try_into().unwrap());

        ZomeCallUnsigned {
            provenance: HoloHash::<Agent>::from_raw_39(val.provenance).unwrap(),
            cell_id: val.cell_id.into(),
            zome_name: ZomeName::new(val.zome_name),
            fn_name: FunctionName::new(val.fn_name),
            cap_secret: cap_secret.map(CapSecret::from),
            payload: ExternIO::from(val.payload),
            nonce: Nonce256Bits::from(nonce),
            expires_at: Timestamp(val.expires_at),
        }
    }
}

#[derive(uniffi::Record)]
pub struct ZomeCallFFI {
    pub cell_id: CellIdFFI,
    pub zome_name: String,
    pub fn_name: String,
    pub payload: Vec<u8>,
    pub cap_secret: Option<Vec<u8>>,
    pub provenance: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
    pub expires_at: i64,
}

impl From<ZomeCall> for ZomeCallFFI {
    fn from(value: ZomeCall) -> Self {
        Self {
            cell_id: value.cell_id.into(),
            zome_name: value.zome_name.0.to_string(),
            fn_name: value.fn_name.into(),
            payload: value.payload.into(),
            cap_secret: value.cap_secret.map(|s| s.as_ref().to_vec()),
            provenance: value.provenance.get_raw_39().to_vec(),
            signature: value.signature.0.to_vec(),
            nonce: value.nonce.into_inner().to_vec(),
            expires_at: value.expires_at.0,
        }
    }
}

#[derive(uniffi::Enum)]
pub enum RoleSettingsFFI {
    UseExisting {
        cell_id: CellIdFFI,
    },
    Provisioned {
        membrane_proof: Option<Vec<u8>>,
        modifiers: Option<DnaModifiersOptFFI>,
    },
}

impl From<RoleSettingsFFI> for RoleSettings {
    fn from(val: RoleSettingsFFI) -> Self {
        match val {
            RoleSettingsFFI::UseExisting { cell_id } => RoleSettings::UseExisting {
                cell_id: cell_id.into(),
            },
            RoleSettingsFFI::Provisioned {
                membrane_proof,
                modifiers,
            } => RoleSettings::Provisioned {
                membrane_proof: membrane_proof
                    .map(|p| std::sync::Arc::new(SerializedBytes::from(UnsafeBytes::from(p)))),
                modifiers: modifiers.map(|m| m.into()),
            },
        }
    }
}
