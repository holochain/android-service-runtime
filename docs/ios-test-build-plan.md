# iOS test build plan

Target: an `ios` branch off `main-0.7` where `holochain-runtime-example` (the
unified in-process `tauri-plugin-holochain` app) builds and runs on the iOS
simulator, and ideally on a device. This is a test build — proving the stack
compiles, the conductor boots, and zome calls execute — not a shippable app.

## Status: running on a physical device (2026-08-28)

Phases 1–4.2 are complete. `holochain-runtime-example` builds for
`aarch64-apple-ios-sim` and `aarch64-apple-ios`, and runs on **both** an
iPhone 17 Pro simulator (iOS 26.5) and a physical **iPhone 12 mini
(iOS 18.7.8)** — conductor boot, hApp install, zome call + signing, and an app
signal, with data persisting across restarts on device (§4.2).

The original blocker — lair's keystore socket exceeding the AF_UNIX path limit
— was fixed rather than worked around: holochain's in-process keystore no
longer binds a socket (§4.0 and "Upstream issues to file → A"). Both iOS path
hacks were deleted as a result.

Simulator log excerpt (device excerpt in §4.2):

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
3. lair's keystore socket exceeded the AF_UNIX path limit under
   `app_data_dir()` on both iOS targets (§4.0). **Now fixed upstream-style**:
   holochain's "in-process" keystore no longer binds a socket, so the path
   length is irrelevant and the conductor boots from the conventional data
   dir. Both iOS `data_dir()` workarounds are gone and mobile is one uniform
   arm again.

> ⚠️ **This branch no longer builds standalone.** The workspace `Cargo.toml`
> carries a `[patch.crates-io]` block pointing at a sibling `../holochain`
> checkout on branch `ios-inproc-keystore`. Without that checkout, `cargo`
> fails to resolve. This is deliberate and temporary — it must not merge to
> `main` until the keystore change is upstream and released. See "Upstream
> issues to file → A".

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
— see §4.0. `data_dir()` now has three mobile arms: `target_os = "android"`,
iOS-device, and iOS-simulator (split on `target_abi = "sim"`, because the
simulator's container path is 70 bytes longer than a device's).

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

### 4.0. Blocker: lair's keystore socket vs. the AF_UNIX path limit — RESOLVED

*Kept in full because the analysis explains why the fix is what it is. The
outcome: holochain's in-proc keystore no longer binds a socket, the workarounds
below were deleted, and the conductor boots from `app_data_dir()`. See
"Upstream issues to file → A" for the patch and its proof.*

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

iOS paths are far longer than that. All figures below are **canonicalized**,
since lair calls `dunce::canonicalize` on the root — on a device that turns
`/var` into `/private/var` and costs 8 bytes; simulator containers live under
`/Users` and are unaffected.

| path | device | simulator |
| --- | --- | --- |
| container root alone | 84 B | 162 B |
| `<container>/socket` | 91 B ✅ | 169 B ❌ |
| `<container>/hc/socket` | 94 B ✅ | 172 B ❌ |
| `<container>/Documents/hc/socket` | 104 B ❌ | 182 B ❌ |
| `app_data_dir()/holochain/socket` | 158 B ❌ | 236 B ❌ |

**The two targets fail for different reasons, and this is the part that is easy
to misread.** A device container is ~78 bytes shorter than a simulator's, so on
a device a short in-container path clears the limit with ~10 bytes to spare —
`Documents/hc` misses by exactly one byte, and anything carrying the bundle id
(28 bytes on its own) is hopeless. In the simulator the container is 162 bytes
before anything is appended, so **nothing under it can ever fit**.

Which inverts the usual expectation: the working simulator build is the *more*
artificial of the two. It only runs because simulator apps are ordinary macOS
processes that can write outside their container (`/tmp/hcex`), an escape a
real device does not permit. The device, with its stricter sandbox but shorter
paths, is the one that can plausibly stay inside the container. So a green
simulator run says nothing about a device, in either direction.

The workarounds this originally shipped — `/tmp/hcex` in the simulator and
`home_dir()/hc` on device — have been **deleted**. Neither was shippable: the
first writes outside the sandbox, the second had ~10 bytes of headroom and was
forced to put conductor data at the container root rather than in `Application
Support`, losing conventional backup semantics.

That last point was the real argument for fixing it upstream rather than tuning
the path: the constraint was never "our directory name is too long", it was
that a whole class of correct locations was unreachable. `data_dir()` is now a
single `#[cfg(mobile)]` arm using `app_data_dir()`, same as Android.

**The socket is avoidable, and that is the real story.** lair ships a genuinely
socket-free in-process server — `InProcKeystore` (`in_proc_keystore.rs`) wires
client to server over in-memory channels and only reads the `?k=` server pubkey
out of `connection_url`; it never calls `get_connection_path()` and never
binds. holochain deliberately bypasses it in
`holochain_keystore::lair_keystore::spawn_lair_keystore_in_proc`:

```rust
// rather than using the in-proc server directly,
// use the actual standalone server so we get the pid-checks, etc
let mut server = StandaloneServer::new(config).await?;
```

So iOS is blocked by an opt-in to pid-checks that are meaningless inside a
single sandboxed app process. holochain already demonstrates the socket-free
pattern in the same crate — `spawn_mem_keystore()` (gated behind `test_utils`)
does `InProcKeystore::new(config, store_factory, passphrase)` then
`new_client()`, using root `"/"`, which proves the path is irrelevant on that
path. Making it persistent instead of in-memory means swapping the factory for
`lair_keystore::create_sql_pool_factory(&config.store_file,
&config.database_salt)` — the same call `StandaloneServer::run` makes
internally, and already a public re-export.

See "Upstream issues to file" below.

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

### 4.2. Device build — DONE (iPhone 12 mini, iOS 18.7.8)

The stack runs on real hardware. Full chain, captured from device syslog:

```
holochain conductor ready; installing app + opening window
main window opened
[ui] env:    OK — direct Tauri IPC — INSTALLED_APP_ID=forum, hasSigner=true
[ui] conn:   OK — connected — appInfo.installed_app_id = forum
[ui] zome:   OK — get_all_posts returned 3 record(s) — zome call + signing OK
[ui] create: OK — create_post OK
[ui] signal: OK — received — type=app zome=posts
```

**`3 record(s)` is the headline.** The simulator always returned 0 on a fresh
data dir; three records means posts written by *earlier launches* were still
there. So the conductor databases and the lair keystore persist across restarts
in the real app container — at `app_data_dir()`, the path that was impossible
before the §4.0 keystore fix. That is a stronger result than the simulator
could give, since a device sandbox is genuinely enforced.

Measurements and observations:

- **Cold-ish boot: 34.0 s** (process start 11:08:13.8 → conductor ready
  11:08:47.9) versus 18.9 s on an M-series Mac — roughly 1.8× slower, as
  expected for an A14 running an interpreter with no serialized module cache.
  Zome call landed 0.85 s after the window opened.
- **No jetsam kill.** Memory limit was 2098 MB and the app held foreground
  priority throughout, despite the 467 MB debug IPA. Worth re-checking with a
  release build, but the debug binary size is not fatal on device.
- **`data_dir()` needed no device-specific arm.** With the socket gone the
  device uses the same `app_data_dir()` as Android and the simulator; the
  `target_abi = "sim"` split and its ~10-byte headroom risk were deleted, and
  the device run confirms that was right.
- **`NSLocalNetworkUsageDescription`** is in the Info.plist, but no
  local-network TCC request was logged — the only TCC traffic was a
  `kTCCServiceMicrophone` request from WKWebView. So either iroh never
  attempted LAN discovery in this session or it does not need the permission
  for what it did. Unresolved, and worth revisiting when testing peer-to-peer.

Networking — partly answered, see §4.2.1 below.

Signing setup (the team ID is **not** committed):

```
# 1. Xcode ▸ Settings ▸ Accounts ▸ + ▸ Apple ID, then select the team and
#    "Manage Certificates…" ▸ + ▸ Apple Development to create a cert.
# 2. Confirm it landed:
security find-identity -v -p codesigning     # expect >=1 valid identity
pnpm tauri ios build --help >/dev/null; pnpm tauri info   # lists certificates
# 3. Team ID is the 10-char code in Xcode ▸ Settings ▸ Accounts, or at
#    developer.apple.com/account ▸ Membership.
export APPLE_DEVELOPMENT_TEAM=XXXXXXXXXX
```

The env var is `APPLE_DEVELOPMENT_TEAM` (the tauri CLI reads exactly this
name); the config equivalent is `bundle > iOS > developmentTeam`. The CLI
injects `DEVELOPMENT_TEAM[sdk=iphoneos*]`, `CODE_SIGN_IDENTITY`,
`CODE_SIGN_STYLE` and `PROVISIONING_PROFILE_SPECIFIER` into the `xcodebuild`
call, so nothing needs baking into `project.yml`. For CI there are
`IOS_CERTIFICATE`, `IOS_CERTIFICATE_PASSWORD` and `IOS_MOBILE_PROVISION`.

On the device itself: iOS 16+ needs Settings ▸ Privacy & Security ▸ **Developer
Mode** enabled, then connect over USB and trust the Mac.

Then build and install — prefer `build` over `dev` for the same embedded-asset
reason as §4.1:

```
pnpm tauri ios build --target aarch64 --debug --export-method debugging
xcrun devicectl device install app --device <udid> <path-to>.app
xcrun devicectl device process launch --device <udid> org.holochain.runtimeexample
```

Gotchas hit on the way, all of which cost time:

- **Xcode's Run button can never work on this project.** It fails with
  `failed to read CLI options: Error when opening the TCP socket: Connection
  refused` — Tauri's "Build Rust Code" phase expects to be launched *by* the
  `tauri` CLI, which runs a local socket to pass it build options. Running
  `xcodebuild` directly hits the same wall. Xcode is only needed once, to
  register the device; all builds go through `pnpm tauri ios build`. (A failed
  Run also makes Xcode silently revert the run destination, which looks like
  "selecting the iPhone won't stick".)
- **`-allowProvisioningUpdates` will not register a new device.** The first
  build failed with "Your team has no devices from which to generate a
  provisioning profile". Registration needs the Xcode GUI once, or adding the
  UDID by hand at developer.apple.com ▸ Devices. After that the CLI is
  self-sufficient.
- **First launch is refused** with "invalid code signature, inadequate
  entitlements or its profile has not been explicitly trusted by the user" —
  that last clause is the real one. On a personal/free team you must trust the
  cert on the phone: Settings ▸ General ▸ VPN & Device Management ▸ Developer
  App ▸ Trust. A free-tier profile also expires in **7 days**.
- **App logs do not reach `devicectl --console`.** `tauri-plugin-log` output
  goes to the unified log, so use `idevicesyslog` (or Console.app). Note
  `devicectl device process launch` on an already-running app just foregrounds
  it — terminate by PID first (`devicectl device info processes` →
  `devicectl device process terminate --pid`) or you will capture nothing.
- **A device build writes your Team ID into `project.pbxproj`.** Reproducible:
  `xcodegen generate` leaves 0 occurrences of `DEVELOPMENT_TEAM`, and a
  subsequent `tauri ios build --target aarch64` leaves 2. Opening the project
  in Xcode does the same, and additionally rewrites the scheme (downgrades its
  `version` and renames `BuildableName`). Since `project.pbxproj` is generated
  from `project.yml`, the fix is to regenerate before committing:

  ```
  cd apps/holochain-runtime-example/src-tauri/gen/apple && xcodegen generate
  ```

  A Team ID is not a secret — it ships inside every distributed app's
  `embedded.mobileprovision` — but committing it couples the repo to one
  developer and guarantees a spurious diff every time anyone else builds. The
  build works fine with the ID supplied only via `APPLE_DEVELOPMENT_TEAM`
  (verified: clean `project.pbxproj` + env var → `** BUILD SUCCEEDED **`), so
  there is no reason to carry it in git.

### 4.2.1. Networking on device — partly answered

The device does real network I/O, which the simulator never managed:

```
TrackedFlow  WiFi udp4  org.holochain.runtimeexample  rx pkts 102  tx pkts 130
TrackedFlow  WiFi tcp4  org.holochain.runtimeexample  rx pkts 38   tx pkts 33
Data Usage   WiFi in/out: 123472/79793 bytes
```

Sustained bidirectional UDP is consistent with iroh QUIC actually moving
traffic. But the same IPv6 relay failure the simulator showed is still present:

```
[noq_udp] sendmsg error: HostUnreachable "No route to host",
          Transmit: { destination: [2a01:4ff:f0:febe::1]:7842, ... }
```

That destination is IPv6, and the test network has no IPv6 route — so this
looks like an environment limitation rather than an iOS one, with IPv4 traffic
succeeding alongside it. **Not conclusive**: the plugin logs non-app crates at
`Warn`, so a successful relay handshake would not be logged, only the failure.
Confirming relay connectivity needs a run at `Info`, and actual peer-to-peer
sync needs two devices. Both still open, along with backgrounding (§4.3).

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

## Upstream issues to file

All line references are against `holochain 0.7.0`, `holochain_keystore 0.7.0`,
`lair_keystore_api 0.7.1`, `lair_keystore 0.7.1` as published on crates.io.

### A. holochain — `spawn_lair_keystore_in_proc` binds a unix socket, making iOS impossible (primary)

*Repo: holochain/holochain. This is the one that actually blocks iOS.*

> **Fixed and proven locally.** A patch exists on branch `ios-inproc-keystore`
> in a sibling `../holochain` checkout (cut from the `holochain-0.7.0` tag),
> wired in through `[patch.crates-io]` in the workspace `Cargo.toml`. With it,
> the conductor boots from the standard `app_data_dir()` — a 229-byte root
> whose socket would have been 236 B — and completes a zome call and signal on
> the simulator, with no socket created. Both iOS `data_dir()` workarounds were
> deleted as a result; mobile is one uniform arm again. See "Proof" below.

`holochain_keystore::lair_keystore::spawn_lair_keystore_in_proc` builds a
`StandaloneServer`, which binds an AF_UNIX socket at `<data_root>/socket`
(`lair_keystore_api::ipc_keystore::raw_ipc` →
`tokio::net::UnixListener::bind`). AF_UNIX paths are capped at ~104 bytes,
while an iOS app-container data dir is 150 B (device) to 162 B (simulator)
before anything is appended. Every iOS-legal data directory therefore fails at
conductor startup with:

```
Lair({"error":"InvalidInput","message":"path must be shorter than SUN_LEN"})
```

The socket is not required. `lair_keystore_api::in_proc_keystore::InProcKeystore`
connects client to server over in-memory channels, reads only the `?k=` pubkey
from `connection_url`, and never binds. holochain already uses that shape in
`spawn_mem_keystore()` (`holochain_keystore/src/lib.rs`, `test_utils`-gated),
which passes root `"/"` — the path is inert on that path. The only difference
for a persistent keystore is the store factory:
`lair_keystore::create_sql_pool_factory(&config.store_file, &config.database_salt)`,
which is a public re-export and is exactly what `StandaloneServer::run` calls
internally.

Ask: offer a socket-free in-proc keystore. The patch below does it
unconditionally, since nothing depended on the socket.

**Proof (branch `ios-inproc-keystore`, one file,
`crates/holochain_keystore/src/lair_keystore.rs`).** Replace the
`StandaloneServer` + connect-back-over-IPC dance with:

```rust
// same guard StandaloneServer::new applies, kept verbatim
tokio::task::spawn_blocking(move || ::lair_keystore::pid_check::pid_check(&config))
    .await.map_err(one_err::OneErr::new)??;

// the same persistent sqlcipher store StandaloneServer would have built
let store_factory = ::lair_keystore::create_sql_pool_factory(
    &config.store_file, &config.database_salt);

let keystore = InProcKeystore::new(config, store_factory, passphrase).await?;
let client = keystore.new_client().await?;
std::mem::forget(keystore);          // mirrors the existing mem::forget(server)

let (s, _) = tokio::sync::mpsc::unbounded_channel();
Ok(MetaLairClient(Arc::new(parking_lot::Mutex::new(client)), s))
```

Note the pid-check is **retained** — `pid_check` is public and callable
directly, so the only behaviour dropped is the socket. That matters for
desktop, where two conductors sharing a lair root is a real scenario; the
earlier framing ("pid-checks have no value") was too glib, and it costs
nothing to keep them.

Compatibility: `connection_url` is still read for the server identity key, so
existing lair configs work untouched. The only visible change on disk is that
no `socket` file appears; `pid_file`, `store_file` and the databases are
unchanged.

Results on an iPhone 17 Pro simulator (iOS 26.5), conductor rooted at the
standard `app_data_dir()` (229 B; its socket would have been 236 B):

```
[ui] zome:   OK — get_all_posts returned 0 record(s) — zome call + signing OK
[ui] create: OK — create_post OK
[ui] signal: OK — received — type=app zome=posts

$ ls "<container>/Library/Application Support/org.holochain.runtimeexample/holochain"
databases  lair-keystore-config.yaml  pid_file  store_file  store_file-shm  store_file-wal
                                      ^^^^^^^^ kept          no `socket` — this is the fix
```

0 SUN_LEN errors; 6/6 `tauri-plugin-holochain` desktop tests still pass.

Caveats for the real PR: this was cut from the `holochain-0.7.0` tag so it
could be `[patch.crates-io]`'d into this 0.7.0 workspace, and needs
forward-porting to `develop` (0.8.0-dev) to be submitted. It has not been run
on a physical iOS device, on Android, or against holochain's own test suite.

### B. lair — `tcp://` connection URLs are documented but not implemented

*Repo: holochain/lair.*

`LairServerConfigInner::connection_url`'s doc comment advertises three schemes:

```rust
/// - `unix:///path/to/unix/socket?k=Yada`
/// - `named_pipe:\\.\pipe\my_pipe_name?k=Yada`
/// - `tcp://127.0.0.1:12345?k=Yada`
```

and `get_connection_scheme()` says `"unix", "named-pipe", or "tcp"`. But
`ipc_keystore/raw_ipc.rs` only handles `"unix"` and `"named-pipe"` — there is no
`TcpListener` anywhere in the crate. Worse, `config::get_connection_path()`
would **panic** on a tcp URL, since it calls
`url.to_file_path().expect("The connection url is invalid …")`.

This matters because loopback TCP is the natural escape hatch from SUN_LEN on
iOS, and the docs suggest it already exists.

Ask: implement tcp loopback binding, or remove it from the docs. If
implemented, note that a loopback listener on iOS may trip the local-network
privacy prompt, so it is a weaker fix than (A).

### C. lair — the generated default `connection_url` is unusable on iOS

*Repo: holochain/lair. Lower priority; fixing (A) makes this cosmetic.*

`LairServerConfigInner::new()` unconditionally derives
`unix://<canonicalize(root_path)>/socket`, with no length validation. On iOS
this silently produces a config that can never bind, and canonicalization adds
a further 8 bytes (`/var` → `/private/var`). Embedders only discover this at
runtime, as an opaque `InvalidInput` from deep inside a bind call.

Note there *is* an escape hatch today, and it is worth documenting either way:
`LairServerConfigInner`'s fields are `pub`, `from_bytes` is `pub`, and
holochain's `get_config` reads an existing `lair-keystore-config.yaml` verbatim
before falling back to generating one. So an embedder can pre-write a config
with a short `connection_url` and a container-resident `store_file`. That works
but every embedder has to rediscover it.

Ask: validate the socket path length in `new()` and fail with a message that
names the problem, and/or document the split-config escape hatch.

## Next steps

1. **Upstream the keystore fix (issue A).** It is written and proven locally on
   `../holochain` branch `ios-inproc-keystore`, but cut from the
   `holochain-0.7.0` tag; it needs forward-porting to `develop` (0.8.0-dev) and
   running against holochain's own test suite before it can be submitted. Until
   it lands and ships in a release, **this branch cannot merge to `main`** —
   the `[patch.crates-io]` block makes the workspace unbuildable without a
   sibling `../holochain` checkout.
2. **Finish the networking question (§4.2.1).** The device moves real UDP
   traffic, but a successful relay handshake is not logged at the current
   level. Re-run with non-app crates at `Info` to confirm relay connectivity,
   then test actual sync between two devices (or device ↔ desktop).
3. **File (B) and (C)** against lair. Secondary now that (A) is solved: (C)
   becomes cosmetic and (B) stops mattering for this use case.
4. **Backgrounding (§4.3)** — never exercised. iOS suspends the app; what the
   conductor does on resume is unknown.
5. **CI job (Phase 5)** to protect what now works.

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
