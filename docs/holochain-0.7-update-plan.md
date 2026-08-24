# Holochain 0.7.0 update plan

Target: the `main-0.7` branch runs against **released holochain 0.7.0** (not the
`0.7.0-dev.*` pre-releases the branch started from).

- **Phase 1 (primary):** the unified `tauri-plugin-holochain` works in a desktop
  build, validated in CI.
- **Phase 2 (secondary):** the unified `tauri-plugin-holochain` works in an
  **Android** build — the NDK cross-compile of the 0.7 stack plus Android
  scaffolding for `holochain-runtime-example`.
- **Phase 3 (deferred):** the two Android service/client crates
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
    ASR app. **This is a breaking change to the FFI/Kotlin surface (Phase 3).**
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
   - Manual smoke: `pnpm run start:example` in the dev shell (webkitgtk 2.52
     with a host-first EGL vendor list on non-NixOS hosts — the host's native
     drivers are tried first, nixpkgs Mesa is the last resort, which drives
     AMD/Intel GPUs in hardware and falls back to software only on NVIDIA).
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

## Phase 2 — Android: unified tauri-plugin-holochain

Goal: `holochain-runtime-example` builds and runs as an Android app (x86_64
emulator + aarch64 device) embedding the in-process conductor. No uniffi, no
Kotlin — the plugin is pure Rust, so this phase is (a) making the 0.7 stack
cross-compile under the NDK and (b) giving the example app an Android target.

### 2.1 NDK toolchain updates — DONE (outcome differed from prediction)

The probe (`cargo ndk -t x86_64 -t arm64-v8a --platform 27 check -p
tauri-plugin-holochain`) surfaced a different blocker than expected:

- **`aws-lc-sys` 0.41 built fine** under cargo-ndk (via iroh's `noq-proto`
  crypto and rustls' default provider) — the feared "iroh on Android NDK" risk
  was a non-event, needing only `cmake` (now in the flake; it had been silently
  leaking in from the host).
- **The real blocker was vendored OpenSSL 3.6** (`libsqlite3-sys` →
  sqlcipher → `openssl-src` 300.6): its `sm4-x86_64.S` uses SM4 AVX
  instructions (`vsm4rnds4`) that NDK r26's clang 17 integrated assembler
  rejects. Verified empirically that clang 18 (r27+) assembles them. Fixed by
  bumping the flake NDK **26.1 → r28.2 (28.2.13676358)**: clang 19, and 16 KB
  page-aligned `.so`s by default (Play requirement for Android 15+ targeting).
- **cargo-ndk poisons host builds**: it exports plain `CC`/`CXX`/`AR` pointing
  at NDK clang, which hijacks *host-targeted* units too (sqlx-macros'
  proc-macro chain builds its own vendored OpenSSL for the host). Fixed by
  exporting `HOST_CC`/`HOST_CXX`/`HOST_AR` in the dev shell — the `cc` crate
  gives these precedence for host units.
- **SDK platform 36 added to the flake** alongside 34: tauri 2.11's bundled
  `:tauri-android` gradle subproject compiles against it, and gradle can't
  auto-install platforms into the read-only nix store.
- Other native deps (ring, libsodium-sys, bundled sqlcipher C, wasmer 7.1
  cranelift) all compiled. Wasmer JIT remains a *runtime* watch item (2.3).
  The tx5/WebRTC stack (the old NDK pain) is gone entirely.

### 2.2 Android scaffolding for the example app — DONE

1. `tauri android init` generated `gen/android` (identifier changed to
   `org.holochain.runtimeexample` — Android package segments can't contain
   `-`); `minSdk` 27 to match the other apps; `package.json` with
   `start:android`/`build:android` added and wired into the pnpm workspace +
   root scripts (`build:holochain-runtime-example`, `start:example-android`).
   Template deviations worth knowing: `ndkVersion` is set in
   `app/build.gradle.kts` (must match the flake — without it AGP can't find
   `llvm-strip` and packages the unstripped Rust `.so`, a ~919 MB APK that
   won't install on a stock emulator; stripped it's ~159 MB), and the tauri
   template's `jniLibs.keepDebugSymbols` block is removed for the same reason.
   The same `ndkVersion` pin was added to the other two apps' gradle configs:
   AGP 8.6's default NDK is 26.1, which the old flake happened to ship — with
   only r28 installed they'd otherwise package unstripped jniLibs silently.
2. Plugin audit: no changes needed — the plugin itself is mobile-clean. The
   *example app* needed `#[cfg_attr(mobile, tauri::mobile_entry_point)]` and a
   per-platform data dir (desktop keeps the throwaway temp dir; mobile uses
   `app.path().app_data_dir()/holochain`, so plugin registration moved from
   the builder into `setup()` where the path resolver is live). Single-window
   direct-IPC needs no tauri `unstable` feature.

### 2.3 Validation

1. **Emulator smoke — DONE (x86_64, API 34 AVD):** full lifecycle verified
   from the app's file log (`logs/holochain-runtime-example.log`): lair
   created + unlocked, conductor ready, forum happ installed, main window
   opened, UI connected over direct Tauri IPC, signed zome call
   (`get_all_posts`), `create_post`, and the app signal received. Wasmer 7.1
   cranelift JIT works on Android x86_64. Known non-fatal issue: iroh can't
   read Android's DNS config (`ndk_context not initialized; call
   install_android_jni_context`) and falls back to Google DNS — follow-up is
   to install the JNI context (the `ndk-context` crate) at app start.
   A dedicated AVD `asr_example_test` (12 G data partition) was created for
   this; the stock 6 G images don't fit the debug APK's extracted lib.
2. **Device check (aarch64)** when available — JIT and iroh sockets behave
   differently on real devices (`cargo ndk -t arm64-v8a check` already
   passes).
3. **CI — DONE:** `build.yml` gained a `build-example-android` job building
   the debug APK (`--debug --target x86_64 --apk`) in the nix shell — debug
   so it needs no release-signing environment. Emulator-connected tests can
   come later with Phase 3's jobs.

## Phase 3 (deferred) — Android: service/client crates + Kotlin libraries

Goal: `tauri-plugin-service` + `tauri-plugin-client` (and beneath them
`runtime-ffi` / `runtime-types-ffi` → uniffi Kotlin bindings →
`libraries/service` / `libraries/client`) build and pass the emulator suites;
both Android apps build.

1. **Cross-compile check.** `pnpm run build:single-target:runtime-ffi
   x86_64-linux-android` (and the aarch64 target) — should be routine once
   Phase 2.1's NDK toolchain work (aws-lc-sys under cargo-ndk) has landed,
   since `runtime-ffi` sits on the same `holochain-conductor-runtime` stack.
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
   builds both APKs. Phase 3 is done when both jobs are green on the
   `main-0.7` PR.

## Open questions / risks

- **Default relay URL** is still the iroh-canary dev relay
  (`use1-1.relay.n0.iroh-canary.iroh.link`); confirm the production relay and
  bootstrap URLs for 0.7 before any release.
- **iroh on Android NDK** (Phase 2.1) is the largest unknown, now concretely
  identified as `aws-lc-sys` (CMake-built C, pulled in by iroh's `noq-proto`
  and rustls) needing to build under cargo-ndk/NDK 26. aws-lc-rs documents
  Android support, so this should be toolchain plumbing rather than a hard
  blocker; if it does fail, the fallback to investigate is forcing rustls's
  `ring` provider through the iroh/kitsune2 feature chain.
- **`restore_from_dht` / `init_properties`** are deliberately not exposed over
  FFI yet; expose later if Android consumers need DHT restore or migration
  properties.
