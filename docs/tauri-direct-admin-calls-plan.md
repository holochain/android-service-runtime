# Plan: ASR additions for unyt-on-ASR (phased, in lockstep with unyt)

Status: proposed
Date: 2026-05-26 (rev. 2026-05-27 — re-scoped to match unyt's simplified phasing)
Builds on: Phase 5 (`docs/tauri-direct-app-calls-plan.md`, the in-process `app_request`
IPC path).
Branch: `feat/tauri-direct-admin-calls`, off `feat/tauri-direct-app-calls`.
Companion to unyt's `../unyt/unyt-admin-access-needs.md` (the unyt-side phasing). **This
doc tracks only the ASR-side additions; the phase numbers match unyt's.**

## Why this is now tiny

unyt drives all admin from its **own in-process Rust `#[tauri::command]`s** calling
`app.holochain()?.runtime().<method>()`. It needs **nothing** from a loopback admin
websocket and **nothing** from an admin-over-IPC surface (capabilities gate JS→Rust, not
Rust→runtime). And unyt's multi-network UX is now **per-window** — a dashboard window
plus one `main_window_builder`-bound window per network entered, which already works on
the *current* ASR (Phase 5). So:

- **Phase 2 (open-mode parity) needs exactly one ASR add: `Runtime::dump_network_stats()`.**
- **Phase 3 (authenticated mode) needs an ASR "controllable-boot" cluster** (sketched
  below; designed in detail when we get there).

Everything I had drafted for an admin-IPC path or trusted multi-app windows is **dropped
or deferred** (see "Removed / deferred" below).

## Trust model (unchanged context, explains the absences)

The webapp UI + DNA are **bundled into the compiled binary** today — no trust boundary;
the windows are fully first-party. A real boundary appears only in the future
"install unknown `.webhapp` files" case, and even then the enforcement primitive is
*pinning* (a window bound to its app via `main_window_builder`, JS-supplied `app_id`
ignored), which Phase 5 already ships. So: **admin stays Rust-only** (no admin IPC to
gate), and there is no `app_id`-per-request feature (it was only ever a trusted-window
convenience, which the per-window UX removes the need for).

## Phase 2 — the single ASR add (build now, lockstep with unyt Phase 2)

### `Runtime::dump_network_stats()` (`crates/runtime/src/runtime.rs`)

`req_admin_api(AdminRequest) -> AdminResponse` already exists (private). Add one public
wrapper — a plain method unyt's Rust calls, no IPC, no command:

```rust
pub async fn dump_network_stats(&self) -> RuntimeResult<ApiTransportStats> {
    match self.req_admin_api(AdminRequest::DumpNetworkStats).await? {
        AdminResponse::NetworkStatsDumped(stats) => Ok(stats),
        fail => Err(RuntimeError::AdminApiBadResponse(fail)),
    }
}
```

- **Return the typed struct, not a string.** unyt's About dialog reads
  `data.transport_stats.backend`, which is
  `ApiTransportStats.transport_stats: TransportStats { backend: String, .. }`; a
  stringified payload would hide it.
- *Verified against pinned `holochain_conductor_api-0.6.1` (`src/admin_interface.rs`):*
  `DumpNetworkStats → AdminResponse::NetworkStatsDumped(kitsune2_api::ApiTransportStats)`.
  The **admin** `DumpNetworkStats` returns `ApiTransportStats` (the
  `transport_stats` + `blocked_message_counts` wrapper); the app-API one returns the
  narrower `TransportStats` — use the admin path. **First step:** confirm the
  re-export path for `ApiTransportStats` (via `holochain`/`holochain_conductor_api` vs.
  a direct `kitsune2_api` dep) and reference it consistently in the signature.

### Non-change to honor

- **Keep `list_apps()` arg-less.** unyt filters by status client-side and several ASR
  Phase 1 callers use the no-arg form. Do **not** add a filter argument or a
  `list_apps_filtered` sibling — it's not wanted.

That is the whole of Phase 2 on the ASR side.

## Phase 3 — controllable-boot cluster (deferred; design when we get there)

unyt's authenticated/secured mode needs ASR to hand over control of the conductor's boot
and key lifecycle. Captured here so we build it together later; **not designed yet**:

- **Boot on unyt's schedule / late passphrase.** Today `Runtime::new_with_network_config`
  takes the passphrase + network config up front. Phase 3 needs the lair password
  supplied late (stronghold / user-prompt flow) rather than at plugin init.
- **Pre-launch hook with lair access.** Before the conductor starts networking, unyt must
  compute + sign hc-auth material using the keystore and inject it into `NetworkConfig`.
  Needs a hook point that has lair access but precedes network start.
- **Restart-keeping-lair.** Re-launch the conductor / re-apply `NetworkConfig` without
  losing or re-unlocking the keystore.
- **`generate_agent_pub_key()`** — only needed for the hc-auth pre-registration case
  (a key minted before any app install). `req_admin_api(AdminRequest::GenerateAgentPubKey)`
  → `AdminResponse::AgentPubKeyGenerated(AgentPubKey)`. (Trivial, but Phase 3 — open mode
  reuses the auto-generated `agent_key` from `list_apps()`.)
- **`export_agent_seed()` + pending-seed-on-boot** — agent-key backup/restore. Pairs with
  the existing `import_key_seed`; needs an "inject seed at next boot" path.

## Removed / deferred (and why)

| Item from earlier drafts | Disposition | Why |
|---|---|---|
| `admin_request` IPC command, denylist, `holochain:admin` capability | **Removed** | unyt does admin in Rust; bundled model has no untrusted JS admin consumer. No admin IPC surface = nothing to gate. |
| Trusted multi-app windows: `AppScope`, `app_id` on `app_request`, `resolve_app` | **Removed** | unyt's multi-network is per-window (bind at creation via `main_window_builder`); no in-place app switching needed. |
| client-js app-transport change (`app_id`, `connect(installedAppId)`, per-app signals) | **Removed** | Follows from the above — per-window uses the existing Phase 5 transport unchanged. **No client-js change in this phase.** |
| `rebind_window` + per-window signal-forwarder lifecycle | **Removed** | unyt explicitly doesn't need rebind; windows bind at creation. (The Phase 5 forwarder-cleanup-on-close follow-up still exists, but is not on unyt's path — defer.) |
| `generate_agent_pub_key()` | **→ Phase 3** | Not needed for open mode (reuse auto-generated key); only the hc-auth pre-registration case. |
| `list_apps_filtered(AppStatusFilter)` | **Dropped** | unyt filters client-side; keep `list_apps()` arg-less. |
| `export_agent_seed` / pending-seed import | **→ Phase 3** | Part of the authenticated-mode key-backup cluster. |

## Phasing & exit criteria

- **Phase 2 (now).** Add `Runtime::dump_network_stats() -> ApiTransportStats`. Unit/
  integration test against the in-process conductor: boot the forum fixture, call it,
  assert `transport_stats.backend` is present and typed. Keep `list_apps()` arg-less.
  *Exit:* unyt's About dialog can read network stats over a direct `runtime()` call; no
  other ASR change; no client-js change.

- **Phase 3 (later, co-designed).** The controllable-boot cluster above. *Exit:* TBD —
  define alongside unyt's Phase 3 (hc-auth, lair-password prompt,
  `complete_hc_auth_and_restart`, agent-key backup/import).

## Lockstep / cross-repo

- **Phase 2** ships with unyt's Phase 2. ASR side = the one method above on
  `feat/tauri-direct-admin-calls`; unyt side = per-window multi-network restructure +
  un-stub onto `runtime()` + delete `AppInterfaceManager`/`get_app_connection_info`/port
  plumbing. **No `holochain-client-js` change** (unlike Phase 5 / earlier drafts).
- The Phase 5 capability gotcha still applies — consumers need the right `core:*`
  permissions for events ([[phase5-direct-tauri-call-path]]).
- **Phase 3** is a joint design effort (the controllable-boot capability spans ASR's boot
  lifecycle and unyt's hc-auth flow); start it only after Phase 2 lands.
