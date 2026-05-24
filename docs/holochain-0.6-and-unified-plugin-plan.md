# Plan: Advance to Holochain 0.6.1 and add a unified (FFI-free) Tauri plugin

Status: proposed
Date: 2026-05-24
Source of the version-advancing work: `../volla-cloud-services` (`HelloVolla/volla-cloud-services`), a fork of this repo.

## Goal

0. Remove all reliance on the darksoil `tauri-plugin-holochain` flake (currently used for its dev shell) by building our own Nix dev shell from first principles for our use-case.
1. Bring over the Holochain-version-advancing work from the `volla-cloud-services` fork (currently `0.6.1-rc.7`), **without** the Volla-specific branding, registry, and CI changes.
2. Complete the update to the now-released **Holochain 0.6.1**.
3. Ensure good test coverage that verifies the whole stack works.
4. Refactor so we can add a **third** Tauri plugin that unifies client + service into a single in-process Rust binary with **no FFI/UniFFI/Kotlin/AIDL** — while still fully supporting the existing separated (client + service) deployment.

## Locked decisions

- **Default network URLs:** use Holochain public infrastructure defaults (not the `*.volla.tech` endpoints the fork uses). Keep the new `relay_url` config field.
- **Port mechanism:** clean re-apply onto a fresh branch off `main`, excluding all Volla-specific noise. Do *not* merge the fork and revert.
- **Unified plugin target:** desktop-first. Android in-process is a follow-up.

## Architecture (current)

The orchestration logic is already pure Rust in `crates/runtime/src/runtime.rs`
(`new`, `setup_app`, `ensure_app_websocket`, `install_app`, `enable_app`,
`disable_app`, `uninstall_app`, `sign_zome_call`, `authorize_app_client`, ...).
Everything above it is thin wrapping:

```
                        ┌─ tauri-plugin-service (Rust shim) ─→ Kotlin HolochainServicePlugin ─┐
separated path:  UI ──→ │                                                                     ├─→ runtime-ffi (UniFFI) ─→ crates/runtime ─→ conductor
                        └─ tauri-plugin-client  (Rust shim) ─→ Kotlin client ─→ AIDL/Binder ──┘   (cross-app on Android)

unified path (new): UI ─→ tauri-plugin-holochain (pure Rust) ───────────────────────────────────→ crates/runtime ─→ conductor
```

The third plugin links `crates/runtime` directly and re-exposes the same surface
as Rust Tauri commands, skipping UniFFI, Kotlin, and AIDL. The window-builder +
holochain-env JS injection in `crates/tauri-plugin-client/src/mobile.rs` (~lines
47-91) is already pure Rust and reusable.

### Dependency on darksoil-studio/tauri-plugin-holochain (p2p-shipyard)

**This repo does NOT depend on darksoil's `tauri-plugin-holochain` as a code/library dependency.**
- No `Cargo.toml` references it. The crates `tauri-plugin-holochain-service` and
  `tauri-plugin-holochain-service-client` are this repo's own plugins.
- `flake.nix` consumes the darksoil input **only for its dev shell**
  (`devShells.holochainTauriAndroidDev`), i.e. the Android + Holochain Tauri build
  toolchain — not linked plugin code. **Phase 0 removes this dev-shell reliance
  entirely**, after which the repo has zero dependency (code or environment) on
  darksoil.
- The unified plugin (Phase 4) is therefore greenfield on top of this repo's own
  `crates/runtime`; it introduces no new darksoil code dependency.

## Phase 0 — Own the Nix dev shell (drop the darksoil flake)

Replace `inputs.tauri-plugin-holochain` (consumed only via
`devShells.holochainTauriAndroidDev`) with our own dev shell defined directly in
`flake.nix`. **Do not copy darksoil's derivation.** Recreate from first principles
the set of things our build/test actually requires, and provide each from `nixpkgs`
+ `holonix`.

What our use-case actually needs (derived from the repo, not from darksoil):
- **Rust toolchain** pinned by `rust-toolchain.toml`, with the Android targets the
  build scripts use: `aarch64-linux-android` (arm64-v8a), `x86_64-linux-android`
  (x86_64), `i686-linux-android` (x86), plus `x86_64-unknown-linux-gnu` for desktop.
  Provide via a rust toolchain that honors `rust-toolchain.toml` (holonix's rust, or
  a rust overlay such as oxalica/fenix). Bump the channel as needed (the fork moved
  to 1.88.0).
- **Holochain dev tools** from `holonix` (Holochain's own flake — kept): `holochain`,
  `hc`, `lair-keystore`, `hc-scaffold` (the last is needed to rebuild fixtures).
- **Android SDK + NDK** via `nixpkgs` `androidenv.composeAndroidPackages`
  (platform-tools, build-tools, a platform API level, cmake, and an NDK). Export
  `ANDROID_HOME` / `ANDROID_SDK_ROOT` and `ANDROID_NDK` (the build scripts read
  `$ANDROID_NDK/toolchains/llvm/...`), plus `ANDROID_NDK_ROOT` / `NDK_HOME` for tools
  that expect those names; put `platform-tools` (adb) on `PATH`.
- **JDK** (for Gradle) — `jdk17` (or the version the gradle plugins require).
- **`cargo-ndk`** (already a package in the current shell) and **`pnpm` / `nodejs`**.
- **Tauri desktop system libraries** (needed for the desktop-first unified plugin in
  Phase 4, and harmless for Android-only work): `webkit2gtk-4.1`, `gtk3`, `libsoup_3`,
  `librsvg`, `openssl`, `pkg-config`; set `PKG_CONFIG_PATH` and the `XDG_DATA_DIRS` /
  `GIO_MODULE_DIR` (glib-networking) bits so the webview and TLS work in-shell.
- **Android emulator + adb** for the local `test:client` / `test:service` runs (CI
  uses an emulator action; locally we want them available).

Implementation notes:
- Keep `holonix` as an input; remove `tauri-plugin-holochain` from `inputs` and from
  `inputsFrom`. Build `devShells.default` from `nixpkgs` packages + the env exports
  above, layered on `holonix.devShells.default`.
- Provide a clear error/setup path for Android SDK license acceptance if
  `androidenv` requires `android_sdk.accept_license = true` (via `nixpkgs` config or
  an overlay in the flake).
- Sanity-check the shell by running the full `npm test` and an Android
  `build:single-target-arch:x86_64` inside `nix develop` before moving on.

Exit criterion: `nix develop` provides a working shell with no `tauri-plugin-holochain`
input; `flake.nix`/`flake.lock` reference only `holonix` (+ `nixpkgs`/`flake-parts`
followed from it) and our own definitions; `npm test` and an Android single-target
build both succeed.

## Phase 1 — Bring over the version-advancing work (target `0.6.1-rc.7`)

Land the Holochain 0.5.6 -> 0.6.1-rc.7 migration and its new features on a fresh
branch off `main`, re-applied by area (clean history).

Bring over:
1. **Workspace deps** (`Cargo.toml`): `holochain`/`holochain_conductor_api`/`holochain_types`
   -> `0.6.1-rc.7`; `lair_keystore_api` -> `0.6.3`. Verify the `hc_uniffi` 0.29.2 pin
   still applies for 0.6.
2. **Flake** (`flake.nix`): `holonix` -> `main-0.6`; regenerate `flake.lock` and
   bump the Android SDK/NDK + rust channel in our Phase 0 shell as 0.6 requires
   (the darksoil input is already gone after Phase 0).
3. **Core API migration** in `crates/runtime/src/runtime.rs`,
   `crates/runtime-ffi/src/runtime.rs`, `crates/runtime-types-ffi/src/types.rs`:
   - `AppInfoStatus` -> `AppStatus` (`Running` -> `Enabled`, tuple `Disabled`,
     drop `Paused`/`PausedAppReason`/`DeletingAgentKey`).
   - `AdminResponse::AppEnabled { app, errors }` -> `AppEnabled(app)`.
   - `AppBundleSource::Bytes(_.into())`.
   - Drop `allow_throwaway_random_agent_key`; add `danger_bind_addr: None`.
   - Drop `RoleSettingsFfi::UseExisting`.
4. **New features** (keep): `import_key_seed` (Lair seed import; adds `sodoken`/`uuid`
   use), `agent_key: Option<Vec<u8>>` on install, `relay_url` plumbed through
   `config.rs`/`autostart.rs`, `RuntimeError::InvalidArguments` (`error.rs`).
5. **Regenerate the Kotlin/AIDL/Parceler layer** to mirror the FFI (`AppStatusFfi`,
   `relayUrl`, `agentKey`). Both existing plugins keep working unchanged.
6. **Test fixtures:** rebuild `crates/runtime/fixtures/forum.happ` and
   `crates/runtime-ffi/fixtures/forum.happ` with the 0.6 scaffold; migrate the
   example zomes (`post.rs` etc.) to `LinkQuery`/`GetStrategy`/`GetOptions` and
   `hdi 0.7` / `hdk 0.6`.
7. **Docs:** adopt the improved Holochain-bump runbook in `DEVELOPMENT.md`.

Explicitly EXCLUDE (Volla-specific):
- App rename (`en.json` `app_title`), `ic_settings.xml` + `build.rs` meta-data.
- `.cargo/config.toml` Nexus registry; Nexus/Maven publishing changes in
  `package.json` + gradle `build.gradle.kts`.
- Self-hosted-runner CI rewrites (`[self-hosted, yolo]`, `ubuntu:24.04` container).
- The `relay2.volla.tech` / `iroh-relay.volla.tech` default *values* (keep the
  `relay_url` field; default to Holochain public infra).

Exit criterion: `nix develop` + `npm test` builds all crates and passes the
client + service tests against the 0.6 conductor on `0.6.1-rc.7`.

## Phase 2 — Complete update to released Holochain 0.6.1

8. Bump `holochain*` `0.6.1-rc.7` -> final `0.6.1` (and lockstep `lair`/`hdk`/`hdi`/
   `holonix main-0.6` point releases). Re-run `nix flake update`.
9. Fix any API drift between rc.7 and 0.6.1 final (most likely around
   `AppStatus` / `InstallAppPayload`).
10. Rebuild fixtures if the hApp/DNA manifest format changed between rc.7 and final.

Exit criterion: same as Phase 1, green on `0.6.1` final.

## Phase 3 — Testing that verifies the whole stack

11. **Inventory & restore coverage:** keep ASR's emulator-based `test:client` /
    `test:service` (the fork dropped the emulator zome-call step). Confirm what the
    existing `runtime` / `runtime-ffi` Rust tests cover.
12. **Runtime-level Rust integration tests** (`crates/runtime`): full lifecycle —
    `new` -> `install_app` -> `enable_app` -> `ensure_app_websocket` ->
    `sign_zome_call` -> real zome call against the forum fixture ->
    `disable`/`uninstall`. This is the shared correctness bar for both the FFI path
    and the future unified plugin.
13. **New-feature tests:** `import_key_seed` round-trips a known seed to a
    deterministic `AgentPubKey`; install with explicit `agent_key` yields a cell
    with that key; `relay_url` honored in network config.
14. **Cross-layer parity test:** the FFI surface returns the same results as the
    runtime surface for the lifecycle, so the unified plugin can be validated to the
    same expectations.
15. **CI:** keep GitHub-hosted runners; ensure the matrix builds the workspace and
    runs the emulator client/service tests on 0.6.1.

Exit criterion: a single `npm test` (or `nix develop -c npm test`) exercises
runtime lifecycle + new features + client/service, green in CI.

## Phase 4 — Third, FFI-free unified plugin (desktop-first)

New `crates/tauri-plugin-holochain` runs the conductor in-process and exposes the
same JS/Tauri surface as `client` + `service` combined, with zero
UniFFI/Kotlin/AIDL — existing two plugins remain fully supported.

16. **Decouple shared, non-FFI pieces** (avoid duplication):
    - Extract the reusable Rust window-builder + holochain-env injection from
      `crates/tauri-plugin-client/src/mobile.rs` into a shared module/crate.
    - Reuse the existing `guest-js` (`holochain-env`, `tauri-commands`) and
      `permissions/default.toml` so the JS client API is identical across plugins.
    - Command-types: recommend splitting a pure `crates/runtime-types` (serde only)
      that both `runtime-ffi` and the unified plugin depend on, rather than reusing
      the uniffi-decorated `runtime-types-ffi` structs.
17. **Create `crates/tauri-plugin-holochain`** depending directly on `crates/runtime`:
    - Hold `Arc<Runtime>` in Tauri managed state (not a `PluginHandle`).
    - Implement `setConfig`/start, `connectSetupApp`, `signZomeCall`, `listApps`,
      install/enable/disable/uninstall, `ensureAppWebsocket` as Rust Tauri commands
      calling `crates/runtime` directly.
    - Drop `#![cfg(mobile)]` so it builds for desktop and Android.
18. **Own the lifecycle Kotlin handled today** (the substantive new work): conductor
    data dir + Lair keystore/passphrase setup; process model = in-process conductor
    instead of a separate Android foreground service + cross-app AIDL. Straightforward
    on desktop; on Android, revisit the foreground-service requirement later.
19. **Dev shell:** add a desktop-only variant to our own flake (from Phase 0) — e.g.
    `devShells.desktop` with just the Tauri desktop libs + rust host target, omitting
    the Android SDK/NDK — so desktop builds don't pull the full Android toolchain.
20. **New example app** (or a feature-flagged variant of `apps/example-client-app`)
    wiring the unified plugin, runnable on **desktop** to prove the FFI-free path
    end to end.
21. **Apply Phase 3 lifecycle tests to the unified plugin** so it is verified to the
    same bar as the FFI path. Document in README/DEVELOPMENT when to choose separated
    (cross-app/Android service) vs. unified (single-binary) deployment.

Exit criterion: a desktop Tauri app using only `tauri-plugin-holochain` installs,
runs, and makes a zome call against the forum fixture; the separated client +
service path still passes all Phase 3 tests unchanged.
