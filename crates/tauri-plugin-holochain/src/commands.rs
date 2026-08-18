use crate::{Error, HolochainExt, Result};
use base64::prelude::*;
use holochain::prelude::AgentPubKey;
use holochain_conductor_runtime_types_ffi::{CellIdFfi, ZomeCallParamsFfi};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime, WebviewWindow};

/// Unsigned zome-call params as sent by the webview zome-call signer
/// (`__HC_ZOME_CALL_SIGNER__`). This is the flat, camelCase shape `@holochain/client`
/// produces — note `cell_id` is split into its two hashes here, matching the JS.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SignZomeCallRequest {
    provenance: Vec<u8>,
    cell_id_dna_hash: Vec<u8>,
    cell_id_agent_pub_key: Vec<u8>,
    zome_name: String,
    fn_name: String,
    cap_secret: Option<Vec<u8>>,
    payload: Vec<u8>,
    nonce: Vec<u8>,
    expires_at: i64,
}

/// Signed zome call returned to the webview signer.
#[derive(Serialize)]
pub(crate) struct SignZomeCallResponse {
    bytes: Vec<u8>,
    signature: Vec<u8>,
}

/// Sign a zome call with the conductor's keystore.
///
/// Invoked by the signer injected into the webview, so `@holochain/client` in
/// the UI can make authenticated zome calls without direct keystore access.
#[tauri::command]
pub(crate) async fn sign_zome_call<R: Runtime>(
    app: AppHandle<R>,
    request: SignZomeCallRequest,
) -> Result<SignZomeCallResponse> {
    // Reuse the runtime-types-ffi conversion to build holochain's ZomeCallParams.
    let params = ZomeCallParamsFfi {
        provenance: request.provenance,
        cell_id: CellIdFfi {
            dna_hash: request.cell_id_dna_hash,
            agent_pub_key: request.cell_id_agent_pub_key,
        },
        zome_name: request.zome_name,
        fn_name: request.fn_name,
        cap_secret: request.cap_secret,
        payload: request.payload,
        nonce: request.nonce,
        expires_at: request.expires_at,
    };

    let signed = app
        .holochain()?
        .runtime()
        .sign_zome_call(params.into())
        .await?;

    Ok(SignZomeCallResponse {
        bytes: signed.bytes.into(),
        signature: signed.signature.0.into(),
    })
}

/// Request to sign an arbitrary payload with a specific agent key.
///
/// `agent_key` is the raw 39-byte `AgentPubKey` of an identity this runtime's
/// keystore already holds the private half of, e.g.
/// [`holochain_conductor_runtime::Runtime::device_agent_key`] or
/// [`holochain_conductor_runtime::Runtime::hc_auth_agent_key`]. There is no
/// default identity: the caller must resolve and supply the exact key it
/// means to sign with, since those are distinct identities and a signature
/// from the wrong one is still valid, just for the wrong key.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SignPayloadRequest {
    agent_key: Vec<u8>,
    payload: Vec<u8>,
}

/// Signed payload returned to the caller.
///
/// Unlike [`SignZomeCallResponse`] (raw bytes, meant for `@holochain/client`'s
/// binary wire format), this signature is base64-encoded: the payload here is
/// caller-defined and typically destined for a JSON body, where a string is
/// what a caller can place directly into a field, not a byte array to encode
/// itself.
#[derive(Debug, Serialize)]
pub(crate) struct SignPayloadResponse {
    /// The raw 64-byte Ed25519 signature, base64-encoded (standard alphabet,
    /// padded).
    signature: String,
}

/// Sign an arbitrary payload with a specific agent key held by the
/// conductor's keystore.
///
/// General-purpose counterpart to [`sign_zome_call`]: that signs one fixed,
/// structured payload; this signs whatever bytes the caller supplies, for any
/// protocol that needs proof of control over a Holochain agent key. See
/// [`SignPayloadRequest`] for how the signing key is chosen.
#[tauri::command]
pub(crate) async fn sign_payload<R: Runtime>(
    app: AppHandle<R>,
    request: SignPayloadRequest,
) -> Result<SignPayloadResponse> {
    let agent_key = AgentPubKey::try_from_raw_39(request.agent_key)
        .map_err(|e| Error::Serialization(format!("invalid agent key: {e}")))?;

    let signature = app
        .holochain()?
        .runtime()
        .sign_payload(agent_key, request.payload)
        .await?;

    Ok(SignPayloadResponse {
        signature: BASE64_STANDARD.encode(signature),
    })
}

/// Serve an App API request for the calling window directly from the in-process
/// conductor — the Tauri-IPC replacement for the app websocket.
///
/// `request` is the msgpack-encoded tagged App API request produced by
/// `@holochain/client`'s Tauri transport; the reply is the msgpack-encoded App
/// API response. The target app is resolved from the calling window's label
/// (bound by [`crate::HolochainPlugin::main_window_builder`]), never from the
/// request itself, so a window can only reach the app it was opened for.
#[tauri::command]
pub(crate) async fn app_request<R: Runtime>(
    webview: WebviewWindow<R>,
    request: Vec<u8>,
) -> Result<Vec<u8>> {
    let label = webview.label().to_string();
    webview
        .holochain()?
        .app_request_bytes(&label, request)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tauri::test::{mock_builder, mock_context, noop_assets};
    use tempfile::TempDir;

    fn build_app(tmp: &TempDir) -> tauri::App<tauri::test::MockRuntime> {
        mock_builder()
            .plugin(crate::init(
                crate::vec_to_locked(vec![]),
                crate::HolochainPluginConfig::new(
                    tmp.path().to_path_buf(),
                    crate::NetworkConfig::default(),
                ),
            ))
            .build(mock_context(noop_assets()))
            .expect("failed to build mock tauri app")
    }

    /// Mirrors `tests/integration.rs`'s `wait_for_ready`: the plugin handle is
    /// managed before the conductor boots, so readiness means the runtime is
    /// populated, not just that the plugin is registered.
    async fn wait_for_ready<R: tauri::Runtime>(app: &tauri::App<R>) {
        let mut waited = Duration::ZERO;
        let step = Duration::from_millis(200);
        while app.holochain().and_then(|p| p.try_runtime()).is_err() {
            assert!(
                waited < Duration::from_secs(60),
                "conductor did not become ready within 60s"
            );
            tokio::time::sleep(step).await;
            waited += step;
        }
    }

    /// End-to-end through the command boundary: raw agent-key bytes in, a
    /// base64 signature out that a JSON body can carry verbatim, and that
    /// signature must be a genuine Ed25519 signature over exactly the
    /// requested payload and key.
    #[test]
    fn sign_payload_returns_a_base64_signature_the_key_owns() {
        let tmp = TempDir::new().unwrap();
        let app = build_app(&tmp);

        tauri::async_runtime::block_on(async move {
            wait_for_ready(&app).await;
            let app_handle = app.handle().clone();
            let agent_key = app_handle.holochain().unwrap().runtime().device_agent_key();
            let payload = b"hello reconnect".to_vec();

            let response = sign_payload(
                app_handle,
                SignPayloadRequest {
                    agent_key: agent_key.get_raw_39().to_vec(),
                    payload: payload.clone(),
                },
            )
            .await
            .expect("sign_payload command should succeed");

            let sig_bytes = BASE64_STANDARD
                .decode(&response.signature)
                .expect("signature must be standard-alphabet, padded base64");
            assert_eq!(sig_bytes.len(), 64, "Ed25519 signatures are 64 bytes");

            let mut pub_key_32 = [0u8; 32];
            pub_key_32.copy_from_slice(agent_key.get_raw_32());
            let mut sig_64 = [0u8; 64];
            sig_64.copy_from_slice(&sig_bytes);
            assert!(
                sodoken::sign::verify_detached(&sig_64, &payload, &pub_key_32),
                "decoded signature must verify against the requested key and payload"
            );
        });
    }

    /// Untrusted bytes from the webview must fail cleanly, not panic the
    /// command task. `AgentPubKey::from_raw_39` (used elsewhere in this
    /// codebase at the FFI boundary) panics on a bad length; this command
    /// uses the fallible `try_from_raw_39` instead precisely to avoid that
    /// here, where the input comes straight off the wire.
    #[test]
    fn sign_payload_rejects_a_malformed_agent_key_instead_of_panicking() {
        let tmp = TempDir::new().unwrap();
        let app = build_app(&tmp);

        tauri::async_runtime::block_on(async move {
            wait_for_ready(&app).await;
            let app_handle = app.handle().clone();

            let result = sign_payload(
                app_handle,
                SignPayloadRequest {
                    agent_key: vec![1, 2, 3],
                    payload: b"payload".to_vec(),
                },
            )
            .await;

            assert!(
                matches!(result, Err(Error::Serialization(_))),
                "a malformed agent key must fail cleanly, not panic: {result:?}"
            );
        });
    }
}
