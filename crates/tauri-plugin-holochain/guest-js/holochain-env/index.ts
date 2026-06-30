/// Injects the env `@holochain/client` needs into a webview opened by
/// `tauri-plugin-holochain`, plus a zome-call signer backed by the plugin's
/// keystore. Two modes:
///   - injectHolochainClientEnv: legacy, connects to the in-process conductor's
///     app *websocket* (`__HC_LAUNCHER_ENV__`).
///   - injectHolochainTauriEnv: direct, routes the App API over Tauri IPC with
///     no websocket (`__HC_TAURI_HOLOCHAIN__`), and bridges conductor signals.

import { encode } from '@msgpack/msgpack';
import { listen } from '@tauri-apps/api/event';
import { type CallZomeRequest, type CallZomeRequestSigned } from '@holochain/client';

/// Build the zome-call signer that signs via the plugin's `sign_zome_call`
/// command. Shared by both injection modes.
function makeZomeCallSigner(pluginName: string) {
  return {
    signZomeCall: async (request: CallZomeRequest): Promise<CallZomeRequestSigned> => {
      const nonce = Uint8Array.from(await crypto.getRandomValues(new Uint8Array(32)));
      const expiresAt = 1e3 * (Date.now() + 3e5);
      const payload = Array.from(encode(request.payload));

      const zomeCall = {
        provenance: Array.from(request.provenance),
        cellIdDnaHash: Array.from(request.cell_id[0]),
        cellIdAgentPubKey: Array.from(request.cell_id[1]),
        zomeName: request.zome_name,
        fnName: request.fn_name,
        capSecret: null,
        payload,
        nonce: Array.from(nonce),
        expiresAt,
      };

      const response = await (window as any).__TAURI_INTERNALS__.invoke(
        `plugin:${pluginName}|sign_zome_call`,
        { request: zomeCall }
      );

      return {
        bytes: Uint8Array.from(response.bytes),
        signature: Uint8Array.from(response.signature),
      } as CallZomeRequestSigned;
    },
  };
}

/// Legacy app-websocket wiring: `@holochain/client` dials `ws://localhost:<port>`
/// and authenticates with the token.
function injectHolochainClientEnv(installedAppId: string, port: number, token: Uint8Array) {
  (window as any).__HC_LAUNCHER_ENV__ = {
    INSTALLED_APP_ID: installedAppId,
    APP_INTERFACE_PORT: port,
    APP_INTERFACE_TOKEN: token,
  };
  (window as any).__HC_ZOME_CALL_SIGNER__ = makeZomeCallSigner('holochain');
}

/// Direct Tauri-IPC wiring: `@holochain/client` routes the App API through the
/// plugin's `app_request` command and receives signals forwarded as
/// `holochain://signal` events — no app websocket.
function injectHolochainTauriEnv(installedAppId: string, pluginName: string) {
  const env = {
    // Empty string means the window is currently unbound (app-less / dashboard);
    // a non-empty id means it is bound to that app. Mutated in place on rebind.
    INSTALLED_APP_ID: installedAppId,
    PLUGIN_NAME: pluginName,
    // Bridge the plugin's per-window signal events to the transport. The Rust
    // side emits the msgpack-encoded conductor Signal as the event payload
    // (a byte array).
    subscribeSignals: (cb: (bytes: Uint8Array) => void): (() => void) => {
      let unlisten: (() => void) | undefined;
      let stopped = false;
      void listen<number[]>('holochain://signal', (event) => {
        cb(Uint8Array.from(event.payload));
      }).then((fn) => {
        if (stopped) fn();
        else unlisten = fn;
      });
      return () => {
        stopped = true;
        unlisten?.();
      };
    },
  };
  (window as any).__HC_TAURI_HOLOCHAIN__ = env;
  (window as any).__HC_ZOME_CALL_SIGNER__ = makeZomeCallSigner(pluginName);

  // Live rebind: after `HolochainPlugin::rebind_window`, the plugin emits
  // `holochain://rebound` with a monotonic `seq` and the new app id (or null when
  // unbound). Apply only when `seq` is newer than the last we applied — so an
  // out-of-order delivery can't leave the env on a stale app while routing and
  // signals are on the latest — then update the env in place and dispatch a DOM
  // `holochain-rebound` event so the host SPA can reconnect `@holochain/client` to
  // the new app (no OS-window recreate, so the window survives).
  let lastReboundSeq = 0;
  void listen<{ seq: number; app_id: string | null }>('holochain://rebound', (event) => {
    if (event.payload.seq <= lastReboundSeq) return;
    lastReboundSeq = event.payload.seq;
    const appId = event.payload.app_id;
    env.INSTALLED_APP_ID = appId ?? '';
    window.dispatchEvent(
      new CustomEvent('holochain-rebound', { detail: { installedAppId: appId } })
    );
  });
}

(window as any).injectHolochainClientEnv = injectHolochainClientEnv;
(window as any).injectHolochainTauriEnv = injectHolochainTauriEnv;
