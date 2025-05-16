use holochain_conductor_api::{
    AppAuthenticationTokenIssued, AppInfo, AppInfoStatus, CellInfo, ProvisionedCell, StemCell,
    ZomeCall,
};
use holochain_types::{
    app::{
        AppBundleError, AppBundleSource, DisabledAppReason, InstallAppPayload, PausedAppReason,
        RoleSettings,
    },
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
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct DurationFfi {
    pub secs: u64,
    pub nanos: u32,
}

impl From<Duration> for DurationFfi {
    fn from(value: Duration) -> Self {
        Self {
            secs: value.as_secs(),
            nanos: value.subsec_nanos(),
        }
    }
}

impl From<DurationFfi> for Duration {
    fn from(val: DurationFfi) -> Self {
        Duration::from_nanos(val.secs * 1000 + val.nanos as u64)
    }
}

#[derive(uniffi::Record)]
pub struct DnaModifiersFfi {
    pub network_seed: String,
    pub properties: Vec<u8>,
    pub origin_time: i64,
    pub quantum_time: DurationFfi,
}

impl From<DnaModifiers> for DnaModifiersFfi {
    fn from(value: DnaModifiers) -> Self {
        Self {
            network_seed: value.network_seed,
            properties: value.properties.bytes().to_owned(),
            origin_time: value.origin_time.0,
            quantum_time: value.quantum_time.into(),
        }
    }
}

#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct DnaModifiersOptFfi {
    pub network_seed: Option<String>,
    pub properties: Option<Vec<u8>>,
    pub origin_time: Option<i64>,
    pub quantum_time: Option<DurationFfi>,
}

impl From<DnaModifiersOptFfi> for DnaModifiersOpt<YamlProperties> {
    fn from(val: DnaModifiersOptFfi) -> Self {
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
pub struct ProvisionedCellFfi {
    pub cell_id: CellIdFfi,
    pub dna_modifiers: DnaModifiersFfi,
    pub name: String,
}

impl From<ProvisionedCell> for ProvisionedCellFfi {
    fn from(value: ProvisionedCell) -> Self {
        Self {
            cell_id: value.cell_id.into(),
            dna_modifiers: value.dna_modifiers.into(),
            name: value.name,
        }
    }
}

#[derive(uniffi::Record)]
pub struct ClonedCellFfi {
    pub cell_id: CellIdFfi,
    pub clone_id: String,
    pub original_dna_hash: Vec<u8>,
    pub dna_modifiers: DnaModifiersFfi,
    pub name: String,
    pub enabled: bool,
}

impl From<ClonedCell> for ClonedCellFfi {
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
pub struct StemCellFfi {
    pub original_dna_hash: Vec<u8>,
    pub dna_modifiers: DnaModifiersFfi,
    pub name: Option<String>,
}

impl From<StemCell> for StemCellFfi {
    fn from(value: StemCell) -> Self {
        Self {
            original_dna_hash: value.original_dna_hash.get_raw_39().to_vec(),
            dna_modifiers: value.dna_modifiers.into(),
            name: value.name,
        }
    }
}

#[derive(uniffi::Enum)]
pub enum CellInfoFfi {
    Provisioned(ProvisionedCellFfi),
    Cloned(ClonedCellFfi),
    Stem(StemCellFfi),
}

impl From<CellInfo> for CellInfoFfi {
    fn from(value: CellInfo) -> Self {
        match value {
            CellInfo::Provisioned(provisioned) => CellInfoFfi::Provisioned(provisioned.into()),
            CellInfo::Cloned(cloned) => CellInfoFfi::Cloned(cloned.into()),
            CellInfo::Stem(stem) => CellInfoFfi::Stem(stem.into()),
        }
    }
}

#[derive(uniffi::Enum, Eq, PartialEq, Debug)]
pub enum PausedAppReasonFfi {
    Error(String),
}

impl From<PausedAppReason> for PausedAppReasonFfi {
    fn from(value: PausedAppReason) -> Self {
        match value {
            PausedAppReason::Error(error) => PausedAppReasonFfi::Error(error),
        }
    }
}

#[derive(uniffi::Enum, Eq, PartialEq, Debug)]
pub enum DisabledAppReasonFfi {
    NeverStarted,
    NotStartedAfterProvidingMemproofs,
    DeletingAgentKey,
    User,
    Error(String),
}

impl From<DisabledAppReason> for DisabledAppReasonFfi {
    fn from(value: DisabledAppReason) -> Self {
        match value {
            DisabledAppReason::NeverStarted => DisabledAppReasonFfi::NeverStarted,
            DisabledAppReason::NotStartedAfterProvidingMemproofs => {
                DisabledAppReasonFfi::NotStartedAfterProvidingMemproofs
            }
            DisabledAppReason::DeletingAgentKey => DisabledAppReasonFfi::DeletingAgentKey,
            DisabledAppReason::User => DisabledAppReasonFfi::User,
            DisabledAppReason::Error(error) => DisabledAppReasonFfi::Error(error),
        }
    }
}

#[derive(uniffi::Enum, Eq, PartialEq, Debug)]
pub enum AppInfoStatusFfi {
    Paused { reason: PausedAppReasonFfi },
    Disabled { reason: DisabledAppReasonFfi },
    Running,
    AwaitingMemproofs,
}

impl From<AppInfoStatus> for AppInfoStatusFfi {
    fn from(value: AppInfoStatus) -> Self {
        match value {
            AppInfoStatus::Paused { reason: paused } => AppInfoStatusFfi::Paused {
                reason: paused.into(),
            },
            AppInfoStatus::Disabled { reason: disabled } => AppInfoStatusFfi::Disabled {
                reason: disabled.into(),
            },
            AppInfoStatus::Running => AppInfoStatusFfi::Running,
            AppInfoStatus::AwaitingMemproofs => AppInfoStatusFfi::AwaitingMemproofs,
        }
    }
}

#[derive(uniffi::Record)]
pub struct AppInfoFfi {
    /// The unique identifier for an installed app in this conductor
    pub installed_app_id: String,
    pub cell_info: HashMap<String, Vec<CellInfoFfi>>,
    pub status: AppInfoStatusFfi,
    pub agent_pub_key: Vec<u8>,
}

impl From<AppInfo> for AppInfoFfi {
    fn from(value: AppInfo) -> Self {
        let mut cell_info: HashMap<String, Vec<CellInfoFfi>> = HashMap::new();
        for entry in value.cell_info.into_iter() {
            let entry_cell_infos: Vec<CellInfoFfi> =
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

#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct AppAuthenticationTokenIssuedFfi {
    pub token: Vec<u8>,
    pub expires_at: Option<i64>,
}

impl From<AppAuthenticationTokenIssued> for AppAuthenticationTokenIssuedFfi {
    fn from(value: AppAuthenticationTokenIssued) -> Self {
        Self {
            token: value.token,
            expires_at: value.expires_at.map(|v| v.0),
        }
    }
}

#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct AppAuthFfi {
    pub authentication: AppAuthenticationTokenIssuedFfi,
    pub port: u16,
}

#[derive(uniffi::Record, Serialize, Deserialize, Clone, Debug)]
pub struct CellIdFfi {
    pub dna_hash: Vec<u8>,
    pub agent_pub_key: Vec<u8>,
}

impl From<CellId> for CellIdFfi {
    fn from(value: CellId) -> Self {
        Self {
            dna_hash: value.dna_hash().get_raw_39().to_vec(),
            agent_pub_key: value.agent_pubkey().get_raw_39().to_vec(),
        }
    }
}

impl From<CellIdFfi> for CellId {
    fn from(val: CellIdFfi) -> Self {
        CellId::new(
            HoloHash::<Dna>::from_raw_39(val.dna_hash).unwrap(),
            HoloHash::<Agent>::from_raw_39(val.agent_pub_key).unwrap(),
        )
    }
}

#[derive(uniffi::Record)]
pub struct ZomeCallUnsignedFfi {
    pub provenance: Vec<u8>,
    pub cell_id: CellIdFfi,
    pub zome_name: String,
    pub fn_name: String,
    pub cap_secret: Option<Vec<u8>>,
    pub payload: Vec<u8>,
    pub nonce: Vec<u8>,
    pub expires_at: i64,
}

impl From<ZomeCallUnsigned> for ZomeCallUnsignedFfi {
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

impl From<ZomeCallUnsignedFfi> for ZomeCallUnsigned {
    fn from(val: ZomeCallUnsignedFfi) -> Self {
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
pub struct ZomeCallFfi {
    pub cell_id: CellIdFfi,
    pub zome_name: String,
    pub fn_name: String,
    pub payload: Vec<u8>,
    pub cap_secret: Option<Vec<u8>>,
    pub provenance: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
    pub expires_at: i64,
}

impl From<ZomeCall> for ZomeCallFfi {
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

#[derive(uniffi::Enum, Serialize, Deserialize, Clone, Debug)]
pub enum RoleSettingsFfi {
    UseExisting {
        cell_id: CellIdFfi,
    },
    Provisioned {
        membrane_proof: Option<Vec<u8>>,
        modifiers: Option<DnaModifiersOptFfi>,
    },
}

impl From<RoleSettingsFfi> for RoleSettings {
    fn from(val: RoleSettingsFfi) -> Self {
        match val {
            RoleSettingsFfi::UseExisting { cell_id } => RoleSettings::UseExisting {
                cell_id: cell_id.into(),
            },
            RoleSettingsFfi::Provisioned {
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

#[derive(uniffi::Record)]
pub struct InstallAppPayloadFfi {
    /// Raw bytes of encoded AppBundle
    pub source: Vec<u8>,
    pub installed_app_id: String,
    pub network_seed: Option<String>,
    pub roles_settings: Option<HashMap<String, RoleSettingsFfi>>,
}

impl TryInto<InstallAppPayload> for InstallAppPayloadFfi {
    type Error = AppBundleError;
    fn try_into(self) -> Result<InstallAppPayload, Self::Error> {
        Ok(InstallAppPayload {
            source: AppBundleSource::Bytes(self.source),
            agent_key: None,
            installed_app_id: Some(self.installed_app_id),
            network_seed: self.network_seed,
            roles_settings: self
                .roles_settings
                .map(|r| r.into_iter().map(|(k, v)| (k, v.into())).collect()),
            ignore_genesis_failure: false,
            allow_throwaway_random_agent_key: false,
        })
    }
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct RuntimeConfigFfi {
    /// Path where conductor data is stored
    pub data_root_path: String,

    /// Network config
    pub network: RuntimeNetworkConfigFfi,
}

#[derive(uniffi::Record, Clone, Debug)]
pub struct RuntimeNetworkConfigFfi {
    /// URL of the bootstrap server
    pub bootstrap_url: String,

    /// URL of the sbd server
    pub signal_url: String,

    /// URLs of ICE servers
    pub ice_urls: Vec<String>,
}

impl Default for RuntimeNetworkConfigFfi {
    fn default() -> Self {
        Self {
            bootstrap_url: "https://bootstrap-0.infra.holochain.org".to_string(),
            signal_url: "wss://sbd.holo.host".to_string(),
            ice_urls: vec!["stun:stun.l.google.com:19302".to_string()],
        }
    }
}
