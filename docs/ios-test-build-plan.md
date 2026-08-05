# iOS test build plan

Target: an `ios` branch off `main-0.7` where `holochain-runtime-example` (the
unified in-process `tauri-plugin-holochain` app) builds and runs on the iOS
simulator, and ideally on a device. This is a test build — proving the stack
compiles, the conductor boots, and zome calls execute — not a shippable app.

## Why this is now possible (and one correction)

iOS forbids JIT compilation (no W+X memory outside WKWebView), so holochain's
default wasm backend (`wasmer-sys-cranelift`, a JIT) cannot execute there. The
interpreter path that unblocks iOS landed in holochain 0.7 — but it is **wasmi,
not WAMR**. The `wasmer_wamr` feature from the 0.5/0.6 era was removed in
0.7.0-dev.23 and replaced by `wasmer-wasmi` (wasmer 7.1's `wasmi` backend,
wasmi 1.1.0). The conductor-config docs say explicitly this is the backend
"recommended … on iOS where compilation on the fly is not permitted"
(`holochain_conductor_api::config::conductor::WasmBackend::Wasmi`).

Caveats that come with that, straight from the holochain repo:

- The backend is labeled **temporary and experimental** ("will be replaced;
  please do not use it as a default" — `docs/developer_setup.md` in holochain).
  Its CI job runs on ubuntu only and is in `allowed-failures`. Fine for a test
  build; expect the feature name to change again.
- **No metering** under wasmi — zome calls have no execution-cost limit, and
  the `wasmer_metering_*` externs are absent from `zome_info`.
- **No serialized-module cache** — wasm source is re-parsed from the DB on
  cold start; only the in-memory moka cache (64 entries, 1h idle) applies.
- Roughly **3x slower** than the JIT (holochain's own test thresholds are bumped
  50ms → 150ms under wasmi).
- `must_get_agent_activity` has a known wasmer bug under wasmi
  ([wasmer#6397](https://github.com/wasmerio/wasmer/issues/6397)); holochain
  excludes some of its own tests for it.

Tauri 2 supports iOS (`tauri ios init/dev/build`), but **only from macOS with
full Xcode** — no Linux path, and nix cannot supply the Apple SDK. Everything in
Phase 1 below is deliberately Linux-runnable; Phases 2–4 need a Mac.

## Branch context

Cut `ios` off `main-0.7` **after** the in-flight Phase 2 Android work
(the `holochain-runtime-example` mobile scaffolding: `mobile_entry_point`,
per-platform data dir, `gen/android`, pnpm wiring) is committed — the iOS build
reuses exactly those example-app changes. The `#[cfg(mobile)]`
`app_data_dir().join("holochain")` branch already covers iOS by construction.

Scope: **unified plugin only.** `tauri-plugin-service` / `tauri-plugin-client`
declare `.ios_path("ios")` for directories that don't exist and reference Swift
bindings that were never written; they are Android-service-specific (iOS has no
foreground-service equivalent anyway) and are not in the example app's
dependency graph. They stay out of any iOS build.

## Phase 1 — wasm backend feature plumbing (Linux, no Mac needed)

Goal: the crates in the example app's graph can be built with either wasm
backend, cranelift stays the default everywhere (Android/desktop unchanged),
and the wasmi path is proven to work on desktop before any Apple toolchain is
involved.

Nothing in the repo currently selects a backend — every crate inherits
holochain's defaults (`encryption`, `schema`, `wasmer-sys-cranelift`). The
switch has to happen at the declaration sites, because default features can't
be subtracted downstream.

1.1. Workspace root: `holochain = { version = "0.7.0", default-features = false }`.
     Crates outside the iOS graph (`runtime-ffi`, `tauri-plugin-service`,
     `tauri-plugin-client`, the ASR app) restore the old behavior with
     `holochain = { workspace = true, default-features = true }`.

1.2. Thread a backend feature pair through the iOS graph
     (`crates/runtime` → `crates/tauri-plugin-holochain` →
     `apps/holochain-runtime-example/src-tauri`; the plugin and the app also
     depend on holochain directly). In `crates/runtime`:

     ```toml
     [dependencies]
     holochain = { workspace = true, features = ["encryption", "schema"] }

     [features]
     default = ["wasm-cranelift"]
     wasm-cranelift = ["holochain/wasmer-sys-cranelift"]
     wasm-wasmi = ["holochain/wasmer-wasmi"]
     ```

     and the same forwarding pattern in the plugin and the app. Holochain 0.7
     emits a `compile_error!` if no backend is enabled, so a
     `--no-default-features` build without `wasm-wasmi` fails loudly rather
     than silently.

     (Fallback if the feature plumbing fights the workspace: 0.7 backends are
     no longer mutually exclusive — compile both and select at runtime via the
     new `wasm_backend` conductor-config option. Rejected as the primary plan
     because it ships dead JIT machinery in the iOS binary and is an App Store
     review risk.)

1.3. Validate the interpreter path on Linux:
     - `cargo test -p tauri-plugin-holochain --no-default-features --features wasm-wasmi`
     - run the desktop example app with wasmi and exercise the forum fixture
       (install, zome call, signal, rebind).
     - `cargo check --workspace --all-targets` and one Android
       single-target build to confirm cranelift defaults are undisturbed.

1.4. Add `aarch64-apple-ios` and `aarch64-apple-ios-sim` to
     `rust-toolchain.toml` targets (harmless on Linux; picked up by rust-overlay
     in the flake and by rustup on the Mac).

## Phase 2 — cross-compile the stack (Mac)

Goal: the example app's Rust lib compiles for `aarch64-apple-ios-sim` before
any Xcode project exists, so toolchain problems surface with fast iteration.

Mac prerequisites: full Xcode (not just CLT), Homebrew, `cocoapods`, `cmake`,
rustup with the toolchain from `rust-toolchain.toml`. The nix dev shell is not
used on the Mac — it can't provide the Apple SDK.

2.1. Compile check:

     ```
     cargo build -p holochain-runtime-example --lib \
       --target aarch64-apple-ios-sim \
       --no-default-features --features wasm-wasmi
     ```

     then the same for `aarch64-apple-ios`. The `staticlib` crate-type Tauri
     iOS needs is already in the example app's Cargo.toml.

2.2. The expected trouble spots are the C deps — the same family that was the
     Android Phase 2.1 blocker:
     - vendored **OpenSSL 3.6** + bundled sqlcipher (`libsqlite3-sys`, via the
       `encryption` feature — and lair pulls sqlcipher regardless, so dropping
       `encryption` doesn't avoid it). Android needed an NDK/clang bump for the
       `sm4` asm; Xcode's clang is ≥17 so this is likely fine, but it's the
       first place to look.
     - **aws-lc-sys** (CMake; rustls default provider + iroh).
     - **ring**, **libsodium-sys-stable**.
     - **wasmi_c_api_impl** — wasmer's wasmi backend is C-based and wasmer 7.1
       uses bindgen, so bindgen/clang must resolve the iOS SDK sysroot (the
       `cc`/`cmake` crates handle this via `xcrun` when `SDKROOT` isn't
       polluted; a plain shell, not the nix shell, matters here).

2.3. Record whatever env/toolchain incantations were needed in this doc, the
     way plan §2.1 did for the NDK.

## Phase 3 — Tauri iOS scaffolding (Mac)

3.1. `pnpm tauri ios init` in `apps/holochain-runtime-example` → `gen/apple`
     (Xcode project + Podfile). Commit it like `gen/android`, minus build
     artifacts.

3.2. Wire the cargo features into the iOS build. `tauri ios dev/build` invoke
     cargo through the generated project, so the cleanest route is a
     `tauri.ios.conf.json` platform override plus passing
     `--no-default-features --features wasm-wasmi` via the tauri CLI's cargo
     args; if that proves awkward, an `ios` cargo alias or a wrapper script in
     package.json (`start:ios`, `build:ios`, matching `start:android`).

3.3. Config: identifier `org.holochain.runtimeexample` is already
     iOS-legal; add `bundle.iOS.minimumSystemVersion` (default 13.0 is fine to
     start) and, for device builds only, a development team
     (`TAURI_APPLE_DEVELOPMENT_TEAM` env var — don't commit a team id).
     Capabilities file already covers `core:default` + `holochain:default`
     with no platform restriction.

3.4. Audit the plugin's window handling on iOS — same open item as Android
     2.2.2: Tauri mobile is single-webview, so `main_window_builder` /
     `bind_window` / `rebind_window` may need the `#[cfg(mobile)]` treatment
     decided there. Whatever Android lands should be re-tested on iOS, not
     assumed.

## Phase 4 — run and validate (Mac)

4.1. Simulator (no signing needed): `pnpm tauri ios dev` →
     - conductor boots; lair in-proc keystore comes up with the passphrase
       flow; data lands under the app-sandbox data dir.
     - install the forum fixture, make a zome call (this is the first actual
       wasmi-on-iOS execution), receive a signal, rebind a window.
     - expect slower zome calls (interpreter) and slower cold installs
       (no module cache) — note timings in this doc.

4.2. Device build: needs an Apple developer team + provisioning.
     `pnpm tauri ios build` (or `ios dev` against a plugged-in device).
     Networking is the thing the simulator can't fully prove: iroh QUIC to the
     relay (still the iroh-canary dev relay — same open risk as the 0.7 plan),
     and whether any LAN peer discovery trips iOS's local-network permission
     (if so, `NSLocalNetworkUsageDescription` in the generated Info.plist).

4.3. Known-limitation notes to record, not fix:
     - iOS suspends the app in background — there is no foreground-service
       equivalent, so the conductor pauses when backgrounded. The in-process
       model is the only viable one on iOS.
     - no metering / no cost limits on zome calls under wasmi.
     - `must_get_agent_activity` may misbehave (wasmer#6397).

## Phase 5 (optional) — CI compile check

A `macos-14` (or later) GitHub runner job that runs the Phase 2.1 compile check
(`aarch64-apple-ios-sim`, `--no-default-features --features wasm-wasmi`) with
plain rustup + Xcode — no nix, no signing, no simulator. This keeps the iOS
target from silently rotting without taking on device testing in CI. Full
`tauri ios build` in CI is out of scope (signing).

## Open questions

- Whether `wasm_backend: wasmi` should also be set explicitly in the conductor
  config the runtime builds (unnecessary when wasmi is the only compiled
  backend, but self-documenting; configuring a backend that isn't compiled in
  panics at startup, so only set it under `#[cfg(target_os = "ios")]`).
- Whether the desktop/Android builds should also gain a runtime wasmi option
  for testing parity, or stay cranelift-only.
- How long `wasmer-wasmi` survives upstream — holochain calls it temporary,
  and its predecessor (`wasmer_wamr`) lasted about two release cycles.
