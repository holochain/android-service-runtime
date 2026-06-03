//! hc-auth: authenticated bootstrap/relay support.
//!
//! For networks that gate bootstrap/relay behind authentication, the conductor
//! must present a base64 "auth material" proving control of an authorized agent
//! key. This module performs that flow against an auth server:
//!
//! 1. get-or-create a persistent Ed25519 agent key in lair,
//! 2. `GET <server>/now` → a base64url challenge payload,
//! 3. sign it with the agent key via lair,
//! 4. `PUT <server>/authenticate` with `{pubKey, payload, signature}` → a status,
//! 5. if authorized, build the base64 auth material to inject into `NetworkConfig`.
//!
//! Ported from the unytco `tauri-plugin-holochain` fork (`feat/hc-auth`), adapted
//! to this crate's [`RuntimeError`] and the holochain `AgentPubKey` re-export.

use crate::{RuntimeError, RuntimeResult};
use base64::prelude::*;
use holochain::prelude::AgentPubKey;
use holochain_keystore::MetaLairClient;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;

fn default_true() -> bool {
    true
}

/// Configuration for the hc-auth flow. `auth_bootstrap`/`auth_relay` select which
/// of the network's auth-material slots the resulting material is written to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HcAuthConfig {
    pub auth_server_url: String,
    #[serde(default = "default_true")]
    pub auth_bootstrap: bool,
    #[serde(default = "default_true")]
    pub auth_relay: bool,
}

/// Result of `PUT /authenticate`, mapped from the HTTP status code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HcAuthStatus {
    Authorized,
    Pending,
    NotRegistered,
    Blocked,
    Failed(String),
}

/// Outcome of [`perform_auth_flow`]: the status plus the agent key (in both
/// holochain and raw-Ed25519-b64url forms) and, when authorized, the auth
/// material to set on the `NetworkConfig`.
#[derive(Debug, Clone)]
pub struct AuthFlowResult {
    pub status: HcAuthStatus,
    pub auth_material: Option<String>,
    pub agent_key: AgentPubKey,
    pub raw_ed25519_b64url: String,
}

fn auth_key_path(holochain_dir: &Path) -> PathBuf {
    holochain_dir.join("hc-auth-agent-key")
}

/// The 32 raw Ed25519 bytes of an agent key, as URL-safe base64 (no padding) —
/// the form the auth server expects for `pubKey`.
pub fn agent_pub_key_to_raw_ed25519_b64url(key: &AgentPubKey) -> String {
    let raw_32: &[u8] = key.get_raw_32();
    BASE64_URL_SAFE_NO_PAD.encode(raw_32)
}

/// Reuse the persisted hc-auth agent key (written to `<dir>/hc-auth-agent-key`)
/// if present, else mint a fresh Ed25519 keypair in lair and persist its pubkey.
pub async fn get_or_create_auth_key(
    keystore: &MetaLairClient,
    holochain_dir: &Path,
) -> RuntimeResult<AgentPubKey> {
    let key_path = auth_key_path(holochain_dir);

    if key_path.exists() {
        if let Ok(stored) = std::fs::read_to_string(&key_path) {
            let trimmed = stored.trim();
            if !trimmed.is_empty() {
                match AgentPubKey::try_from(trimmed) {
                    Ok(key) => {
                        log::info!("Reusing persisted hc-auth agent key");
                        return Ok(key);
                    }
                    Err(e) => {
                        log::warn!("Failed to parse persisted hc-auth key, generating new: {e:?}");
                    }
                }
            }
        }
    }

    log::info!("Generating new hc-auth agent key via Lair");
    let agent_pub_key = keystore
        .new_sign_keypair_random()
        .await
        .map_err(RuntimeError::Lair)?;

    let key_b64 = format!("{}", agent_pub_key);
    if let Err(e) = std::fs::write(&key_path, &key_b64) {
        log::error!("Failed to persist hc-auth agent key: {e}");
    }

    Ok(agent_pub_key)
}

/// `GET <server>/now` → base64url challenge payload.
pub async fn fetch_challenge(auth_server_url: &str) -> RuntimeResult<String> {
    let url = format!("{}/now", auth_server_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| RuntimeError::HcAuth(format!("GET /now failed: {e}")))?;

    if !resp.status().is_success() {
        return Err(RuntimeError::HcAuth(format!(
            "GET /now returned {}",
            resp.status()
        )));
    }

    resp.text()
        .await
        .map_err(|e| RuntimeError::HcAuth(format!("GET /now body read failed: {e}")))
}

/// Sign the challenge payload with `agent_key` via lair; returns the signature as
/// URL-safe base64 (no padding).
pub async fn sign_challenge(
    keystore: &MetaLairClient,
    agent_key: &AgentPubKey,
    payload_b64url: &str,
) -> RuntimeResult<String> {
    let payload_bytes = BASE64_URL_SAFE_NO_PAD
        .decode(payload_b64url)
        .map_err(|e| RuntimeError::HcAuth(format!("Invalid payload base64url: {e}")))?;

    let mut pub_key_32 = [0u8; 32];
    pub_key_32.copy_from_slice(agent_key.get_raw_32());

    let signature = keystore
        .lair_client()
        .sign_by_pub_key(pub_key_32.into(), None, Arc::from(payload_bytes.as_slice()))
        .await
        .map_err(RuntimeError::Lair)?;

    Ok(BASE64_URL_SAFE_NO_PAD.encode(&signature.0[..]))
}

/// Build the base64 auth material (`base64(JSON{pubKey,payload,signature})`) the
/// conductor injects into bootstrap/relay requests.
pub fn build_auth_material(
    pubkey_b64url: &str,
    payload_b64url: &str,
    signature_b64url: &str,
) -> String {
    let auth_body = serde_json::json!({
        "pubKey": pubkey_b64url,
        "payload": payload_b64url,
        "signature": signature_b64url,
    });
    BASE64_STANDARD.encode(auth_body.to_string().as_bytes())
}

/// `PUT <server>/authenticate` with the signed challenge → [`HcAuthStatus`].
pub async fn try_authenticate(
    auth_server_url: &str,
    pubkey_b64url: &str,
    payload_b64url: &str,
    signature_b64url: &str,
) -> RuntimeResult<HcAuthStatus> {
    let url = format!("{}/authenticate", auth_server_url.trim_end_matches('/'));
    let auth_body = serde_json::json!({
        "pubKey": pubkey_b64url,
        "payload": payload_b64url,
        "signature": signature_b64url,
    });

    let client = reqwest::Client::new();
    let resp = client
        .put(&url)
        .header("Content-Type", "application/octet-stream")
        .body(auth_body.to_string())
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| RuntimeError::HcAuth(format!("PUT /authenticate failed: {e}")))?;

    match resp.status().as_u16() {
        200 => Ok(HcAuthStatus::Authorized),
        202 => Ok(HcAuthStatus::Pending),
        401 => Ok(HcAuthStatus::NotRegistered),
        403 => Ok(HcAuthStatus::Blocked),
        other => Err(RuntimeError::HcAuth(format!(
            "PUT /authenticate unexpected status: {other}"
        ))),
    }
}

/// Run the full hc-auth flow. Network/auth-server failures are returned as
/// `Ok(AuthFlowResult { status: Failed(..), .. })` (not `Err`) so the caller can
/// still bring the conductor up (e.g. open mode / retry later) while surfacing
/// the status.
pub async fn perform_auth_flow(
    keystore: &MetaLairClient,
    config: &HcAuthConfig,
    holochain_dir: &Path,
) -> RuntimeResult<AuthFlowResult> {
    let agent_key = get_or_create_auth_key(keystore, holochain_dir).await?;
    let raw_ed25519_b64url = agent_pub_key_to_raw_ed25519_b64url(&agent_key);

    log::info!(
        "hc-auth: Using agent key {}, raw Ed25519: {}",
        agent_key,
        raw_ed25519_b64url
    );

    let payload_b64url = match fetch_challenge(&config.auth_server_url).await {
        Ok(p) => p,
        Err(e) => {
            log::warn!("hc-auth: Could not reach auth server: {e}");
            return Ok(AuthFlowResult {
                status: HcAuthStatus::Failed(format!("Auth server unreachable: {e}")),
                auth_material: None,
                agent_key,
                raw_ed25519_b64url,
            });
        }
    };

    let signature_b64url = sign_challenge(keystore, &agent_key, &payload_b64url).await?;

    let status = match try_authenticate(
        &config.auth_server_url,
        &raw_ed25519_b64url,
        &payload_b64url,
        &signature_b64url,
    )
    .await
    {
        Ok(s) => s,
        Err(e) => {
            log::warn!("hc-auth: Authentication request failed: {e}");
            return Ok(AuthFlowResult {
                status: HcAuthStatus::Failed(format!("{e}")),
                auth_material: None,
                agent_key,
                raw_ed25519_b64url,
            });
        }
    };

    let auth_material = if status == HcAuthStatus::Authorized {
        let material = build_auth_material(&raw_ed25519_b64url, &payload_b64url, &signature_b64url);
        log::info!("hc-auth: Key authorized, auth material generated");
        Some(material)
    } else {
        log::info!("hc-auth: Key status = {:?}, no auth material", status);
        None
    };

    Ok(AuthFlowResult {
        status,
        auth_material,
        agent_key,
        raw_ed25519_b64url,
    })
}
