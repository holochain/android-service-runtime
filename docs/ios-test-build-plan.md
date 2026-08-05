# iOS test build plan

Target: an `ios` branch off `main-0.7` where `holochain-runtime-example` (the
unified in-process `tauri-plugin-holochain` app) builds and runs on the iOS
simulator, and ideally on a device. This is a test build — proving the stack
compiles, the conductor boots, and zome calls execute — not a shippable app.

## Status: done on the simulator (2026-08-05)

Phases 1–4.1 are complete. `holochain-runtime-example` builds for
`aarch64-apple-ios-sim` and `aarch64-apple-ios`, and on an iPhone 17 Pro
simulator (iOS 26.5) the conductor boots, the forum fixture installs, and the
webview makes a zome call over direct Tauri IPC:

```
[ui] env:    OK — direct Tauri IPC — INSTALLED_APP_ID=forum, hasSigner=true
[ui] conn:   OK — connected — appInfo.installed_app_id = forum
[ui] zome:   OK — get_all_posts returned 0 record(s) — zome call + signing OK
[ui] create: OK — create_post OK
[ui] signal: OK — received — type=app zome=posts
```

That is the first wasm execution under wasmi on iOS. Cold start on an empty
data dir: keystore up → conductor ready + hApp installed in **18.9 s**, first
zome call **+1.2 s** after the webview connected.

Three things differed from the plan and are written up in the phases below:

1. The backend could **not** be a cargo feature pair — it had to become a
   target cfg (§1). `tauri ios dev/build` has `--features` but no
   `--no-default-features`, so a feature-based default would have linked the
   JIT into the iOS binary with no way to switch it off.
2. The C/toolchain deps were a **non-issue** (§2) — the opposite of the Android
   experience. The real Phase 3 blocker was a **missing
   `SystemConfiguration.framework`** in the generated Xcode project (§3).
3. The conductor cannot run anywhere under the iOS app container because
   lair's keystore socket exceeds the AF_UNIX path limit (§4). This is the one
   finding that still needs a real fix before anything ships.

Environment used: macOS 26 (Darwin 25.5.0, arm64), Xcode 26.6 (iOS SDK 26.5),
rustup toolchain 1.95.0, Node/pnpm from Homebrew, tauri-cli 2.5.0. No nix.

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
full Xcode** — no Linux path, and nix cannot supply the Apple SDK. Phase 1 as
finally implemented is Linux-runnable (it is only Cargo.toml edits); Phases 2–4
need a Mac. All of the work recorded here was done on a Mac.

## Branch context

Done: `ios` was cut off `main-0.7` after the Android work landed (8f976ea), and
the iOS build reuses those example-app changes — `mobile_entry_point`,
`gen/android`-style scaffolding, pnpm wiring.

One correction to the original assumption: the `#[cfg(mobile)]`
`app_data_dir().join("holochain")` branch did **not** cover iOS by
construction. It compiles, but the conductor cannot boot from that path at all
— see §4.0. `data_dir()` now has separate `target_os = "android"` and
`target_os = "ios"` arms.

Scope: **unified plugin only.** `tauri-plugin-service` / `tauri-plugin-client`
declare `.ios_path("ios")` for directories that don't exist and reference Swift
bindings that were never written; they are Android-service-specific (iOS has no
foreground-service equivalent anyway) and are not in the example app's
dependency graph. They stay out of any iOS build.

## Phase 1 — wasm backend selection (done, but not as a feature pair)

Goal: the iOS build gets wasmi and nothing else; cranelift stays the default
everywhere else (Android/desktop unchanged).

Confirmed against the published crate: holochain 0.7.0's defaults really are
`["encryption", "schema", "wasmer-sys-cranelift"]`, and `wasmer-wasmi` exists.

**The planned feature pair does not work.** `tauri ios dev` and `tauri ios
build` expose `-f/--features` but have **no `--no-default-features`**, and the
cargo invocation is buried in the generated Xcode project's "Build Rust Code"
phase (`tauri ios xcode-script`), so there is nowhere to pass it. A
feature-based `default = ["wasm-cranelift"]` would therefore always link the
JIT into the iOS binary. Making `default` empty instead breaks
`cargo check --workspace`, which cannot pass per-package features.

So the backend is selected by **target cfg** in `crates/runtime/Cargo.toml`:

```toml
[target.'cfg(not(target_os = "ios"))'.dependencies]
holochain = { workspace = true, features = ["encryption", "schema", "wasmer-sys-cranelift"] }

[target.'cfg(target_os = "ios")'.dependencies]
holochain = { workspace = true, features = ["encryption", "schema", "wasmer-wasmi"] }
```

Doing it once in `crates/runtime` — the deepest crate declaring `holochain` —
is enough, because cargo unifies features across the whole graph. The workspace
root sets `holochain = { version = "0.7.0", default-features = false }`; the
plugin and the example app just carry `features = ["encryption", "schema"]`
with no backend. `runtime-ffi` inherits cranelift through `crates/runtime`.

This is strictly better than the feature pair: it needs no flags at any entry
point (tauri CLI, plain cargo, Xcode, CI) and cannot be forgotten. The cost is
that a desktop wasmi build is no longer expressible — see "Open questions".

Verified with `cargo tree -e features -i holochain --target …`:

| target | backend |
| --- | --- |
| `aarch64-apple-darwin` | `wasmer-sys-cranelift` |
| `aarch64-linux-android` | `wasmer-sys-cranelift` |
| `aarch64-apple-ios` | `wasmer-wasmi` |
| `aarch64-apple-ios-sim` | `wasmer-wasmi` |

and confirmed end to end: the iOS build log contains **zero** references to
cranelift.

1.3. Interpreter validated on desktop *before* the restructure, using the
     feature-pair version that existed at the time
     (`--no-default-features --features wasm-wasmi`): the macOS desktop app
     booted, installed the forum fixture, and reported
     `zome: OK … zome call + signing OK`, `create: OK`, `signal: OK`. So wasmi
     executes holochain wasm correctly, independent of iOS.

     Regressions after the restructure: `cargo check --workspace --all-targets`
     passes, and the cranelift targets resolve as in the table above. A real
     Android build still needs the Linux/nix environment (no NDK on this Mac),
     so only the dependency resolution was re-checked here.

1.4. `aarch64-apple-ios` and `aarch64-apple-ios-sim` added to
     `rust-toolchain.toml`. rustup installs them on macOS and skips them
     elsewhere.

## Phase 2 — cross-compile the stack (done, no incantations needed)

Mac prerequisites, as predicted: full Xcode (not just CLT), Homebrew,
`cocoapods`, `cmake`, rustup. The nix dev shell is not used on the Mac.

2.1. Both targets build with **no feature flags** (the target cfg from §1 does
     the selection) and **no environment setup at all**:

     ```
     cargo build -p holochain-runtime-example --lib --target aarch64-apple-ios-sim
     cargo build -p holochain-runtime-example --lib --target aarch64-apple-ios
     ```

     ~1m20s and ~1m each on an M-series Mac, reusing host build scripts and
     proc-macros from a prior desktop build. Output is a ~1.5 GB unstripped
     debug `staticlib` per target (compare the Android 919 MB `.so`).

     Build the UI first — `cargo build` runs `tauri::generate_context!()`,
     which fails if `apps/holochain-runtime-example/ui/dist` doesn't exist.

2.2. **Every predicted C-dep trouble spot was a non-issue.** openssl-src
     3.6.2, `ring`, `aws-lc-sys` 0.41, `libsodium-sys-stable`, and
     `wasmi_c_api_impl` 1.1.0 all built clean on the first attempt. Xcode's
     clang and the `cc`/`cmake` crates resolve the iOS sysroot through `xcrun`
     without help. There is no iOS analogue of the Android NDK/clang-18 SM4
     blocker or the `HOST_CC`/`HOST_CXX`/`HOST_AR` pinning.

2.3. Incantations required: **none**. Two caveats worth knowing:

     - Run plain `cargo` from a shell where `SDKROOT` is unset. Under
       `xcodebuild` that variable *is* set, which changes the fingerprint of
       host-targeted build-script units (e.g. sqlx-macros' vendored OpenSSL)
       and forces them to rebuild against the iOS sysroot. It compiled anyway
       here, but it churns the cache — the same class of problem Android hit
       with `HOST_CC`.
     - The iOS **simulator runtime is not installed with Xcode**. `xcrun
       simctl list runtimes` was empty; `xcodebuild -downloadPlatform iOS`
       fetches it (8.52 GB).

## Phase 3 — Tauri iOS scaffolding (done; one real blocker)

3.1. `pnpm tauri ios init` in `apps/holochain-runtime-example` generated
     `src-tauri/gen/apple` (xcodegen `project.yml` + `.xcodeproj` + Podfile).
     It auto-installs `libimobiledevice` and `xcodegen` via Homebrew. The
     generated `.gitignore` already excludes `Externals/` and `build/`, so
     only 33 files are tracked out of a 3.1 GB directory.

3.2. **No feature wiring was needed** — see §1. The `--no-default-features`
     route the plan sketched does not exist on the tauri CLI, which is exactly
     why the backend moved to a target cfg. `start:ios` / `build:ios` scripts
     were added to the app's package.json and `start:example-ios` to the root,
     matching the Android naming.

3.3. Config: `bundle.iOS.minimumSystemVersion` set to `13.0` (matches the
     generated `deploymentTarget`). Identifier `org.holochain.runtimeexample`
     was already iOS-legal. Capabilities needed no change. No development team
     is committed; simulator builds don't need one.

3.4. **`SystemConfiguration.framework` must be added by hand — this was the
     Phase 3 blocker.** The link fails with ~50 undefined `SC*` symbols
     (`_SCDynamicStoreCreate`, `_kSCPropNetDNSServerAddresses`,
     `_kSCNetworkInterfaceType*`, …) referenced from `netdev`,
     `system_configuration`, and `hickory_resolver` — iroh's interface and DNS
     discovery.

     Cause: the Rust side builds as a `staticlib`, so its
     `#[link(kind = "framework")]` directives never reach Xcode's link step,
     and `tauri ios init`'s template doesn't list the framework. Fix — add to
     `gen/apple/project.yml` under the target's `dependencies:` and regenerate:

     ```yaml
     - sdk: SystemConfiguration.framework
     ```
     ```
     cd apps/holochain-runtime-example/src-tauri/gen/apple && xcodegen generate
     ```

     `xcodegen` runs only at `init`, not on every `dev`/`build`, so the
     regenerated `.xcodeproj` is committed. **Re-running `tauri ios init`
     rewrites `project.yml` and drops this line** — re-add and regenerate.

3.5. The plugin's window handling needed **no** iOS-specific changes.
     `main_window_builder` / `bind_window` / `rebind_window` contain no
     `cfg(mobile)` branches at all and worked as-is, same as on Android.

## Phase 4 — run and validate

### 4.0. Blocker: lair's keystore socket vs. the AF_UNIX path limit

The conductor could not boot anywhere under the iOS app container:

```
Holochain conductor setup failed:
  Lair({"error":"InvalidInput","message":"path must be shorter than SUN_LEN"})
```

Holochain's "in-process" keystore is not socket-free. `holochain_keystore::
spawn_lair_keystore_in_proc` deliberately runs a full `StandaloneServer`
("rather than using the in-proc server directly, use the actual standalone
server so we get the pid-checks") which **binds a unix domain socket**, then
connects to it. Lair derives that socket from the data root
(`connection_url = unix://<canonicalized data_root>/socket`), and AF_UNIX paths
are capped at ~104 bytes.

iOS paths are far longer than that:

| | length |
| --- | --- |
| simulator container | 162 B |
| simulator container + `…/Library/Application Support/<id>/holochain/socket` | 236 B |
| device equivalent | 150 B |
| **AF_UNIX limit** | **~104 B** |

So this is not "pick a shorter subdirectory" — **no** path under
`app_data_dir()` can hold the socket, on device or simulator. A bare device
container plus `/socket` is ~82 B and would fit, but nothing with
`Library/Application Support/<bundle-id>` in it does.

Test-build workaround (in `apps/holochain-runtime-example/src-tauri/src/lib.rs`,
commented there): iOS uses `/tmp/hcex`. That works in the simulator, where apps
can write outside their container. It is **not** shippable — on a real device
`/tmp` is not app-writable, and the data is neither sandboxed nor backed up.

A proper fix belongs upstream and looks cheap: `LairServerConfigInner` already
keeps `connection_url`, `pid_file`, and `store_file` as independent fields, so
the socket could live on a short path while the keystore database stays in the
container. Alternatively lair could `chdir` and bind a relative path. Either
way it needs a lair/holochain change, or a runtime API that lets callers set a
lair root separate from the conductor data root.

### 4.1. Simulator — done

With the framework (§3.4) and the data dir (§4.0) sorted, the full chain runs
on an iPhone 17 Pro simulator (iOS 26.5): conductor boot, forum fixture
install, zome call + signing, and an app signal — the log excerpt is at the top
of this document.

Cold start on an empty data dir: keystore up → conductor ready + hApp installed
in **18.9 s**; first zome call **+1.2 s** after the webview connected. Slow, as
predicted for an interpreter with no serialized-module cache, but a one-off per
install rather than per call.

**Use `tauri ios build`, not `tauri ios dev`, for validation.** In `dev` the
webview loads the frontend from a dev server and iOS blocks it:

```
Failed to request http://127.0.0.1:1430/: … did you grant local network
permissions? … Settings > Privacy & Security > Local Network
```

`simctl privacy` has no `local-network` service to grant, and `--no-dev-server`
just moves the failure to `tauri://localhost` (dev mode doesn't embed assets).
What works is a bundle with embedded assets:

```
pnpm tauri ios build --target aarch64-sim --debug --ci
xcrun simctl install "iPhone 17 Pro" \
  src-tauri/gen/apple/build/arm64-sim/holochain-runtime-example.app
xcrun simctl launch "iPhone 17 Pro" org.holochain.runtimeexample
# logs (the app's stdout does not reach `simctl launch --console-pty`):
xcrun simctl spawn "iPhone 17 Pro" log stream --level debug \
  --predicate 'subsystem == "org.holochain.runtimeexample"' --style compact
```

### 4.2. Device build — not done

Needs an Apple developer team + provisioning, which this machine has none of
(`tauri ios build` warns "No code signing certificates found"). Still open, and
still the only way to prove the networking questions: iroh QUIC to the relay,
and whether LAN peer discovery trips the local-network permission (if so,
`NSLocalNetworkUsageDescription` in the generated Info.plist). Note that §4.0
must be solved first — `/tmp` is not writable on a device.

The simulator already shows iroh failing to reach the dev relay
(`sendmsg error: … HostUnreachable` to the iroh-canary relay), the same open
risk carried over from the 0.7 plan.

### 4.3. Known limitations (recorded, not fixed)

- iOS suspends the app in background — there is no foreground-service
  equivalent, so the conductor pauses when backgrounded. The in-process model
  is the only viable one on iOS.
- No metering / no cost limits on zome calls under wasmi.
- `must_get_agent_activity` may misbehave (wasmer#6397).
- Debug staticlib is ~1.5 GB per target; a release/stripped build matters
  before any real packaging (Android hit the same thing at 919 MB).

## Phase 5 (optional) — CI compile check

Not done. A `macos-14` (or later) GitHub runner job running the Phase 2.1
compile check with plain rustup + Xcode — no nix, no signing, no simulator:

```
cargo build -p holochain-runtime-example --lib --target aarch64-apple-ios-sim
```

(no feature flags — see §1). This keeps the iOS target from silently rotting
without taking on device testing in CI. Worth extending to `tauri ios build
--target aarch64-sim --debug`, which needs no signing and would also catch a
dropped `SystemConfiguration.framework` (§3.4) — a plain `cargo build` will
not, since the missing symbols only surface at Xcode's link step. Full device
`tauri ios build` stays out of scope (signing).

## Next steps

1. **Fix the lair socket path (§4.0).** This gates everything else on iOS —
   device builds included, since `/tmp` is not writable there. Most likely an
   upstream lair/holochain change, or a `crates/runtime` API that separates the
   lair root from the conductor data root.
2. **Device build (§4.2)** once (1) is done and a developer team is available —
   the only way to test iroh QUIC and LAN discovery.
3. **CI job (Phase 5)** to protect what now works.

## Open questions

- Whether `wasm_backend: wasmi` should also be set explicitly in the conductor
  config the runtime builds (unnecessary when wasmi is the only compiled
  backend, but self-documenting; configuring a backend that isn't compiled in
  panics at startup, so only set it under `#[cfg(target_os = "ios")]`).
- The target-cfg backend selection (§1) means **a desktop wasmi build is no
  longer expressible**, so wasmi can only be exercised on iOS. That capability
  was used once, to validate the interpreter before touching Apple tooling, and
  it would be worth having again for debugging. Restoring it needs a feature
  that can *suppress* the cranelift arm, which cargo cannot express — target
  tables can't reference `cfg(feature = ...)`. The realistic option is
  compiling both backends and selecting via holochain 0.7's runtime
  `wasm_backend` config on non-iOS targets only.
- How long `wasmer-wasmi` survives upstream — holochain calls it temporary,
  and its predecessor (`wasmer_wamr`) lasted about two release cycles.
