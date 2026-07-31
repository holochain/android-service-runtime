# Holochain 0.7.0 update plan

Target: the `main-0.7` branch runs against **released holochain 0.7.0** (not the
`0.7.0-dev.*` pre-releases the branch started from).

- **Phase 1 (primary):** the unified `tauri-plugin-holochain` works in a desktop
  build, validated in CI.
- **Phase 2 (secondary):** the two Android service/client crates
  (`tauri-plugin-service`, `tauri-plugin-client`) and the Kotlin libraries they
  feed (`libraries/service`, `libraries/client`) work in the Android build,
  validated in CI.

## Branch context

`main-0.7` = `main-0.6` (holochain 0.6.3, window-rebind, direct Tauri IPC)
+ the rebased "Update to Holochain 0.7 (off develop)" commit
+ the bump from 0.7.0-dev.28 to released 0.7.0.

## Already done on this branch

Rust side compiles clean (`cargo check --workspace --all-targets`: 0 warnings;
`make static` passes) against released crates:

- Pins: `holochain`/`holochain_conductor_api`/`holochain_types` 0.7.0,
  `holochain_keystore` 0.7.0, `kitsune2_api` 0.5.0, `lair_keystore_api` 0.7.1,
  `holochain_serialized_bytes` 0.0.57, hdk 0.7.0 / hdi 0.8.0 (example app).
  Cargo.lock unified on stable (kitsune2 family all 0.5.0; the dev-era
  iroh 1.0.0-rc.0 pin is gone, resolves to iroh 1.0.2).
- Removed the now-unused `[patch.crates-io]` wasmer `fix-x86` fork (0.7 uses
  wasmer 7.1).
- flake: holonix input `main-0.6` → `main-0.7`; dev shell ships holochain 0.7.0.
  `nodejs_20` → `nodejs_22` (20 is EOL and refused by current nixpkgs).
- API changes absorbed:
  - `InstallAppPayload.restore_from_dht` (new): always `false`; not exposed over
    FFI yet.
  - `AppStatus::AwaitingRestore` / `AppStatus::Unrecoverable(CellId, reason)`
    (new): added to `AppStatusFfi` (reason carried as a rendered string).
  - `NetworkConfig` lost `signal_url` + `webrtc_config` (tx5/WebRTC transport
    removed in favor of iroh): `signal_url` and `ice_urls` removed from
    `RuntimeNetworkConfig`, `RuntimeNetworkConfigFfi`, the FFI mappers, and the
    ASR app. **This is a breaking change to the FFI/Kotlin surface (Phase 2).**
  - `RuntimeError` variants boxed (`ConductorError`, `AdminResponse`) —
    clippy `result_large_err` under rust 1.95.
  - Forum fixture ported to hdk 0.7.0 / hdi 0.8.0 and repacked with `hc` 0.7.0
    (both `crates/*/fixtures/forum.happ` copies). The 0.7 `Action` restructure
    (struct + `ActionData` enum) required porting the coordinator's signal
    emission; the scaffolded per-op validation boilerplate (permissive TODO
    stubs throughout) was replaced by an explicit accept-all `validate` — this
    fixture exercises the runtime, not DHT validation.
  - `test_install_with_foreign_key_fails_genesis` updated: 0.7 rejects a
    non-keystore agent key up front with `AgentKeyNotInKeystore` instead of
    failing genesis with an empty source-chain read.

## Phase 1 — desktop: unified tauri-plugin-holochain

Goal: `holochain-runtime-example` (desktop Tauri app embedding
`tauri-plugin-holochain`) installs, runs, and rebinds apps against 0.7.0.

1. **JS client to released 0.21.** DONE — all seven `package.json`s moved from
   the `file:../../../holochain-client-js` dev checkout (which was at
   0.21.0-rc.1) to the released `^0.21.0` from npm; the pnpm workspace
   lockfile regenerated (pnpm 11 build-script approvals recorded in
   `pnpm-workspace.yaml`). The rebuilt `dist-js/holochain-env/index.min.js`
   came out byte-identical (the client import is type-only), confirming the
   shipped bundle is current — the staleness-guard test agrees.
2. **Local validation.**
   - `cargo test -p holochain-conductor-runtime` and `-p
     holochain-conductor-runtime-ffi` (conductor lifecycle, install/enable,
     zome-call signing, hc-auth, seed export).
   - `cargo test -p tauri-plugin-holochain` — integration tests cover boot,
     app setup, direct-IPC admin/app calls, signals, and window rebind.
   - `pnpm run test:example` — builds the desktop example against the plugin.
   - Manual smoke: `pnpm run start:example` in the dev shell (webkit pin).
3. **CI validation.**
   - DONE — `make integration-test` now also runs
     `cargo test -p tauri-plugin-holochain` and `pnpm run test:example`, so
     the desktop plugin suite and example build are gated by `test.yml` on
     PRs to `main-*`.
   - The `static` target (fmt + clippy + `git diff --exit-code`) already gates
     the dist-js staleness guard output.
   - Remaining: land these changes via a PR against `main-0.7` and require
     `test.yml` + `build.yml` green. The stale draft #140 (old
     `asr-main-0.7` lineage) is superseded by `main-0.7`.

## Phase 2 — Android: service/client crates + Kotlin libraries

Goal: `tauri-plugin-service` + `tauri-plugin-client` (and beneath them
`runtime-ffi` / `runtime-types-ffi` → uniffi Kotlin bindings →
`libraries/service` / `libraries/client`) build and pass the emulator suites;
both Android apps build.

1. **Cross-compile check.** `pnpm run build:single-target:runtime-ffi
   x86_64-linux-android` (and the aarch64 target) — first build of holochain
   0.7's iroh/QUIC transport stack under cargo-ndk. Risk item: iroh-native
   deps on the NDK; this is the step that surfaces it.
2. **Kotlin surface updates.** Regenerating the uniffi bindings changes the
   public API:
   - `RuntimeNetworkConfigFfi` loses `signalUrl` + `iceUrls` (breaking).
   - `AppStatusFfi` gains `AwaitingRestore` + `Unrecoverable`.
   Update the handwritten mirrors and tests: `Parcelables.kt`, `Parcelers.kt`,
   `Json.kt`, the two `InvokeTypes.kt`, and
   `libraries/client/src/androidTest` (`JsonTest`, `ParcelablesTest`).
3. **Apps.** `apps/android-service-runtime` (already updated for the config
   change) and `apps/example-client-app` (bump its own Cargo.lock, re-run its
   tryorama tests against the released client stack).
4. **Docs.** Regenerate dokka (`pnpm run build:doc`) — the checked-in docs
   still describe `signalUrl`/`iceUrls`.
5. **CI validation.** `test.yml`'s `test-kotlin-libraries` job already does
   exactly this pipeline (build FFI single-target → publish client to
   mavenLocal → build service → lint → emulator
   `connectedDebugAndroidTest` for client and service), and `build.yml`
   builds both APKs. Phase 2 is done when both jobs are green on the
   `main-0.7` PR.

## Open questions / risks

- **Default relay URL** is still the iroh-canary dev relay
  (`use1-1.relay.n0.iroh-canary.iroh.link`); confirm the production relay and
  bootstrap URLs for 0.7 before any release.
- **iroh on Android NDK** (Phase 2 step 1) is the largest unknown; no
  workaround identified yet if it fails.
- **`restore_from_dht` / `init_properties`** are deliberately not exposed over
  FFI yet; expose later if Android consumers need DHT restore or migration
  properties.
