# Plan: direct Tauri app-call path (drop the loopback app-websocket)

Status: proposed
Date: 2026-05-25
Builds on: Phase 4 of `docs/holochain-0.6-and-unified-plugin-plan.md` (the in-process
`crates/tauri-plugin-holochain`). Think of this as **Phase 5**.

## Goal

In the unified (in-process) plugin, the webview's `@holochain/client` should talk to
the conductor through **Tauri IPC**, not a loopback websocket. Today the plugin runs
the conductor in-process but still `attach_app_interface`s a real TCP websocket and
injects `__HC_LAUNCHER_ENV__` so `@holochain/client` dials `ws://localhost:<port>`
(`crates/tauri-plugin-holochain/src/lib.rs:90-114`). That loopback socket is pure
overhead when the conductor is in the same process.

Add a module to `../holochain-client-js` that **detects the Tauri context and routes
App API calls through the Tauri surface instead of opening a websocket**. The websocket
path stays fully intact for the separated (client + service) deployment and for any
non-Tauri consumer.

## Why this is a small change (architecture facts)

1. **The conductor already exposes an in-process app-API dispatch.**
   `holochain::conductor::api::AppInterfaceApi::new(conductor_handle)
   .handle_request(installed_app_id, Ok(app_request)) -> AppResponse`
   (`holochain-0.6.1/src/conductor/api/api_external/app_interface.rs:34`) is the exact
   twin of the `AdminInterfaceApi` the runtime already drives in
   `crates/runtime/src/runtime.rs:337` (`req_admin_api`). It routes `AppInfo`,
   `CallZome`, `CreateCloneCell`, `DumpNetwork*`, `AgentInfo`, etc. — the whole App API.

2. **The client-js transport boundary is tiny.** `AppWebsocket` only ever touches its
   transport through two surfaces (`src/api/app/websocket.ts`):
   - `this.client.request(taggedReq) -> Promise<taggedRes>` (every requester, line 609-625)
   - `this.client.on("signal", cb)` (constructor, line 243)
   `WsClient` (`src/api/client.ts`) is the *only* place that does websocket I/O. Swap
   the transport object and every method (`callZome`, `appInfo`, `createCloneCell`,
   clone cells, network dumps, countersigning, signals) works unchanged.

3. **The wire payload already is the serde form of the holochain types.** `WsClient`
   sends `encode({ type: tag, value })` (msgpack) and that decodes directly into
   `AppRequest`; the response decodes from `AppResponse`. So a Tauri transport can move
   the *same msgpack bytes* through `invoke()` and reuse holochain's own
   `AppRequest`/`AppResponse` serde with **zero parallel type definitions**.

4. **`connect()` already branches on environment.** `AppWebsocket.connect`
   (`src/api/app/websocket.ts:254`) inspects `getLauncherEnvironment()` to rewrite the
   URL. Adding a "Tauri-direct" branch ahead of it is the natural seam.

5. **Signals have a public subscription.** `Conductor::subscribe_to_app_signals(app_id)
   -> broadcast::Receiver<Signal>` (`holochain-0.6.1/src/conductor/conductor.rs:3204`)
   is a `pub fn` (the enclosing private module does not hide an inherent `pub` method).
   The websocket interface forwards exactly this stream
   (`.../interface/websocket.rs:352`); we forward it over a Tauri channel instead.

## Design

```
websocket path (unchanged):  @holochain/client ─→ WsClient ─ws://localhost:port→ conductor app interface
tauri-direct path (new):     @holochain/client ─→ TauriAppTransport ─invoke()→ plugin app_request ─→ AppInterfaceApi ─→ conductor
```

The unit of transfer is the msgpack-encoded tagged `{ type, value }` — identical bytes
on both paths; only the pipe differs.

### A. holochain-client-js — the "separate module" (no breaking API change)

1. **`src/environments/tauri.ts`** (new — the detector):
   - `isTauriHolochain(): boolean` — true iff `window.__TAURI_INTERNALS__` **and**
     `window.__HC_TAURI_HOLOCHAIN__` are present.
   - `getTauriHolochainEnv(): { installedAppId, pluginName } | undefined`.
   - Mirrors the shape of `src/environments/launcher.ts`.

2. **`src/api/app/tauri-transport.ts`** (new — the transport). A class that
   `extends Emittery` and implements the slice of `WsClient` that `AppWebsocket` uses:
   - `request(taggedReq)`: `encode(taggedReq)` →
     `invoke("plugin:<pluginName>|app_request", { appId, request })` passing the bytes
     as an `ArrayBuffer` → receive response bytes →
     `decode(bytes, { mapKeyConverter })` using the **same** `Uint8Array`→base64
     `mapKeyConverter` `WsClient.handleResponse` uses (`src/api/client.ts:396-411`) so
     HoloHash map keys deserialize identically → return the tagged `{ type, value }`.
   - subscribes to the plugin's signal channel (Tauri event or `Channel`), decodes each
     payload the same way `WsClient.registerMessageListener` decodes a `RawSignal`
     (`src/api/client.ts:256-286`), and `this.emit("signal", ...)`.
   - `authenticate()` → no-op (Tauri IPC is the trust boundary, see security note);
     `close()` → unsubscribe; `emitSignal()` → optional.
   - `catchError`/`promiseTimeout`/`requesterTransformer` from `common.ts` are reused
     untouched.

3. **Relax the transport type** (`src/api/app/websocket.ts`): extract a minimal
   `AppClientTransport` interface (`request(req): Promise<Tagged<...>>` plus Emittery
   `on`/`off`) and change `AppWebsocket`'s `client` field from `WsClient` to that
   interface. `WsClient` already satisfies it — no runtime change.

4. **Branch `AppWebsocket.connect`** (`src/api/app/websocket.ts:254`): if
   `isTauriHolochain()`, construct a `TauriAppTransport` (skip `WsClient.connect`, skip
   the token `authenticate`), then run the same `app_info` request and
   `new AppWebsocket(...)` as today. Tauri-direct is checked **before** the launcher-env
   websocket rewrite so it wins when both are present. `AdminWebsocket` is intentionally
   left websocket-only (admin is not exposed to the webview).

5. Export the new module from `src/index.ts`; add an e2e/unit test stub.

The `__HC_ZOME_CALL_SIGNER__` host-signer flow is **unchanged**: `callZomeTransform`
still signs in JS via the injected signer (the existing `sign_zome_call` command), then
sends the signed `CallZome` through `request()` like any other call.

### B. tauri-plugin-holochain — the Tauri surface

6. **Runtime passthroughs** (`crates/runtime/src/runtime.rs`):
   - `pub async fn handle_app_request(&self, app_id: InstalledAppId, req: AppRequest)
     -> RuntimeResult<AppResponse>` — wraps
     `AppInterfaceApi::new(self.conductor.clone()).handle_request(app_id, Ok(req))`
     (sibling of `req_admin_api`).
   - `pub fn subscribe_to_app_signals(&self, app_id) -> broadcast::Receiver<Signal>`
     — wraps the conductor method.

7. **New command `app_request`** (`crates/tauri-plugin-holochain/src/commands.rs`):
   - Signature takes the window + raw request bytes and returns raw bytes
     (`tauri::ipc::Request`/`Response`, `application/octet-stream`), avoiding a base64
     round-trip.
   - `decode` bytes → `AppRequest` with the codec holochain's websocket uses
     (msgpack via `holochain_serialized_bytes`); dispatch `runtime.handle_app_request`;
     `encode` `AppResponse` → bytes.
   - **App binding (replaces token scoping):** keep a managed `Mutex<HashMap<window
     label, InstalledAppId>>` populated by `main_window_builder`. The command resolves
     `app_id` from the calling window's label, not from a JS-supplied argument, so a
     window can only reach the app it was built for.
   - Register in `build.rs` `COMMANDS` and add `allow-app-request` to
     `permissions/default.toml`.

8. **Signal forwarding**: when `main_window_builder` binds a window to an app, spawn a
   task: `subscribe_to_app_signals(app_id)`, and forward each `Signal` to that window
   (`app.emit_to(label, "holochain://signal", bytes)` or a `tauri::ipc::Channel`),
   serialized in the same shape `@holochain/client` decodes as a `RawSignal`. Drop the
   task when the window closes.

9. **New "direct" window mode** (`crates/tauri-plugin-holochain/src/lib.rs:90`):
   - Instead of `ensure_app_websocket` + injecting `__HC_LAUNCHER_ENV__` (port/token),
     inject `__HC_TAURI_HOLOCHAIN__ = { INSTALLED_APP_ID, pluginName: "holochain" }`
     and keep injecting `__HC_ZOME_CALL_SIGNER__`. No `attach_app_interface`, no
     loopback port.
   - Update `guest-js/holochain-env` and the built
     `dist-js/holochain-env/index.min.js` to add the `injectHolochainTauriEnv` helper
     (sibling of the existing `injectHolochainClientEnv`).
   - Keep the websocket-injection path behind a `WindowOptions` flag for fallback/debug;
     default to direct.

### C. Encoding decision

Move msgpack bytes through Tauri IPC and reuse holochain's `AppRequest`/`AppResponse`
serde on the Rust side and `@msgpack/msgpack` on the JS side — the same bytes the
websocket carries. No new DTOs, no JSON, no divergence from the websocket path. (First
implementation step: confirm `holochain_serialized_bytes::{encode,decode}` round-trips
the `@msgpack/msgpack` output for `AppRequest`/`AppResponse`, including the
`Uint8Array`-keyed maps — this is the one codec assumption to nail down up front.)

## Security note

The websocket path is scoped by a per-app auth token. The Tauri-direct path replaces
that with: (a) the command derives `app_id` from the **window label**, never from JS
input, and (b) Tauri's capability/permission system gates `plugin:holochain|app_request`
to the intended windows. Zome calls remain signed by the in-process keystore via the
existing `sign_zome_call` command. Document this trust model in the plugin README.

## Phasing & exit criteria

- **P5.1 — Rust dispatch.** Add `handle_app_request` + a unit/integration test in
  `crates/tauri-plugin-holochain/tests/integration.rs`: install the forum fixture, build
  an `AppRequest::AppInfo` and `AppRequest::CallZome` (signed via `sign_zome_call`),
  dispatch through `handle_app_request`, assert the `AppResponse` matches the websocket
  path. *Exit:* request/response parity with the app websocket, no socket involved.

- **P5.2 — JS transport.** Land `tauri.ts` + `tauri-transport.ts` + the `connect`
  branch in holochain-client-js; relax the transport type. Unit-test `request()`
  encode/dispatch/decode against a mocked `invoke`. *Exit:* `AppWebsocket` drives
  `callZome`/`appInfo`/`createCloneCell` through a mock Tauri transport unchanged.

- **P5.3 — Signals.** Wire `subscribe_to_app_signals` → Tauri channel →
  `TauriAppTransport` Emittery. *Exit:* a zome that emits a signal reaches
  `appWs.on("signal", ...)` in a direct-mode webview.

- **P5.4 — End to end.** Switch the unified plugin's `main_window_builder` to direct
  mode by default; run the Phase 4 desktop example app, confirm install + zome call +
  signal with **no** app interface port attached (verify via conductor admin
  `ListAppInterfaces` / netstat). *Exit:* the desktop app works end to end over Tauri
  IPC only; the separated client + service path and the websocket fallback still pass
  all existing tests.

## Future optimization (out of scope here)

Fold signing into a single `call_zome` command (sign + dispatch in Rust) to remove the
JS-side signing round-trip. Deferred to keep this change a true drop-in for the existing
`AppWebsocket` signing flow.
