/// Inject Holochain Client magic variables into window
/// Intended to be used by the HolochainPlugin.kt after getting an app websocket.

import { encode } from '@msgpack/msgpack';
import { type CallZomeRequest, type CallZomeRequestSigned } from '@holochain/client';

function injectHolochainClientEnv(appId: string, appWebsocketPort: number, appToken: Uint8Array) {
  (window as any).__HC_LAUNCHER_ENV__ = {
    APP_INTERFACE_PORT: appWebsocketPort,
    INSTALLED_APP_ID: appId,
    APP_INTERFACE_TOKEN: appToken
  };

  (window as any).__HC_ZOME_CALL_SIGNER__ = {
    signZomeCall: async (request: CallZomeRequest): Promise<CallZomeRequestSigned> => {
        const nonce = Uint8Array.from(await crypto.getRandomValues(new Uint8Array(32)));
        const expiresAt = 1e3*(Date.now()+3e5);
        const payload = Array.from(encode(request.payload));

        const zomeCallUnsigned = {
            provenance: request.provenance,
            cellIdDnaHash: request.cell_id[0],
            cellIdAgentPubKey: request.cell_id[1],
            zomeName: request.zome_name,
            fnName: request.fn_name,
            capSecret: null,
            payload,
            nonce,
            expiresAt,
        };
        const response = await (window as any).__TAURI_INTERNALS__.invoke("plugin:holochain-service-consumer|sign_zome_call", zomeCallUnsigned);
        const zomeCallSigned = {
            provenance: request.provenance,
            cell_id: request.cell_id,
            zome_name: request.zome_name,
            fn_name: request.fn_name,
            cap_secret: null,
            payload,
            nonce,
            expires_at: expiresAt,
            signature: Uint8Array.from(response.signature),
        } as CallZomeRequestSigned;

        return zomeCallSigned;
    }
};
}

(window as any).injectHolochainClientEnv = injectHolochainClientEnv;

// Define function to install app, get app websocket, and inject magic config variables
async function setupApp(appId: string, appBundleBytes: number[], networkSeed: string) {
  if (window.location.origin !== 'http://tauri.localhost') return;

  // Check if happ is installed
  const { installed } = await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|is_app_installed', { appId });
  
  // Install happ if not already
  if(!installed) {
    await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|install_app', { appId, appBundleBytes, membraneProofs: {}, networkSeed });
  }
  
  // Setup app websocket
  const { port, token } = await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|ensure_app_websocket', { appId });

  // Inject magic configuration variables used by @holochain/client 
  injectHolochainClientEnv(appId, port, token);
}

(window as any).setupApp = setupApp;