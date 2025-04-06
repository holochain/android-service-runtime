/// Inject Holochain Client magic variables into window
/// Intended to be used by the HolochainPlugin.kt after getting an app websocket.

import { encode } from '@msgpack/msgpack';
import { type CallZomeRequest, type CallZomeRequestSigned } from '@holochain/client';

function injectHolochainClientEnv(installedAppId: string, port: number, token: Uint8Array) {
  (window as any).__HC_LAUNCHER_ENV__ = {
    INSTALLED_APP_ID: installedAppId,
    APP_INTERFACE_PORT: port,
    APP_INTERFACE_TOKEN: token
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
async function setupApp(installedAppId: string, source: number[], networkSeed: string, enableApp: boolean) {
  if (window.location.origin !== 'http://tauri.localhost') return;

  await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|connect');

  // Check if happ is installed
  const { installed } = await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|is_app_installed', { installedAppId });
  
  // Install happ if not already
  if(!installed) {
    await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|install_app', { installedAppId, source, roleSettings: {}, networkSeed });

    // Enable App
    if(enableApp) {
      await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|enable_app', { installedAppId });
    }

    // Hacky workaround attempting to ensure that `window.__HC_LAUNCHER_ENV__` is defined *before* the app UI is loaded.
    // This is NOT 100% reliable. It assumes that the `ensure_app_websocket` and `injectHolochainClientEnv` calls,
    // will take *less* time to complete than the app UI loading.
    //
    // If the app UI loads first, then `window.__HC_LAUNCHER_ENV__` will still not be defined before the holochain client connects.
    //
    // See https://github.com/holochain/android-service-runtime/issues/74
    window.location.reload();
  }
  
  // Setup app websocket
  const { port, authentication: { token } } = await (window as any).__TAURI_INTERNALS__.invoke('plugin:holochain-service-consumer|ensure_app_websocket', { installedAppId });

  // Inject magic configuration variables used by @holochain/client 
  injectHolochainClientEnv(installedAppId, port, token);
}

(window as any).setupApp = setupApp;