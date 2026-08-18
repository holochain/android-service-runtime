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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SignPayloadRequest {
    agent_key: Vec<u8>,
    payload: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub(crate) struct SignPayloadResponse {
    signature: String,
}

/// Deliberately outside the plugin's `default` permission set: [`sign_zome_call`]
/// signs the hash of a well-formed `ZomeCallParams`, so what it produces is only
/// usable as the zome call it describes, while this signs bytes the caller chose,
/// which carry no such domain separation. A capability must name
/// `allow-sign-payload` itself.
#[tauri::command]
pub(crate) async fn sign_payload<R: Runtime>(
    app: AppHandle<R>,
    request: SignPayloadRequest,
) -> Result<SignPayloadResponse> {
    // `from_raw_39`, used elsewhere at the FFI boundary, panics on a bad length or a
    // mismatched hash prefix, and this input comes straight off the wire.
    let agent_key = AgentPubKey::try_from_raw_39(request.agent_key)
        .map_err(|e| Error::Serialization(format!("invalid agent key: {e}")))?;

    let signature = app
        .holochain()?
        .try_runtime()?
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
    webview.holochain()?.app_request_bytes(&label, request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{build_app, plugin_config, wait_for_ready};
    use tauri::test::{mock_builder, mock_context, noop_assets};
    use tempfile::TempDir;

    #[test]
    fn sign_payload_returns_a_base64_signature_the_key_owns() {
        let tmp = TempDir::new().unwrap();
        let app = build_app(tmp.path());

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
            let sig_64: [u8; 64] = sig_bytes
                .as_slice()
                .try_into()
                .expect("Ed25519 signatures are 64 bytes");
            let pub_key_32: [u8; 32] = agent_key.get_raw_32().try_into().unwrap();
            assert!(
                sodoken::sign::verify_detached(&sig_64, &payload, &pub_key_32),
                "decoded signature must verify against the requested key and payload"
            );
        });
    }

    #[test]
    fn sign_payload_rejects_a_malformed_agent_key_instead_of_panicking() {
        let tmp = TempDir::new().unwrap();
        let app = build_app(tmp.path());

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

    #[test]
    fn sign_payload_reports_not_ready_before_the_conductor_starts() {
        let tmp = TempDir::new().unwrap();
        let app = mock_builder()
            .plugin(crate::init_deferred(plugin_config(tmp.path())))
            .build(mock_context(noop_assets()))
            .expect("failed to build mock tauri app");

        let result = tauri::async_runtime::block_on(sign_payload(
            app.handle().clone(),
            SignPayloadRequest {
                agent_key: AgentPubKey::from_raw_32(vec![0u8; 32])
                    .get_raw_39()
                    .to_vec(),
                payload: b"payload".to_vec(),
            },
        ));

        assert!(
            matches!(result, Err(Error::NotReady)),
            "invoking before the conductor boots must return NotReady: {result:?}"
        );
    }
}
