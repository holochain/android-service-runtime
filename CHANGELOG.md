# Unreleased

# 0.2.2
- bump to holochain 0.5.6
- fix example client app connection bug
- fix invalid bootstrap url
- move settings link from homepage to system settings

# 0.2.1
- bump to holochain 0.5.4
- fix infrastructure urls

# 0.2.0
- CI to publish kotlin libraries to Maven Central, triggered by a release tag.
- Specify ICE Servers in `RuntimeConfigFfi`
- Add support for translations, with only english locale written.
- CI to publish tauri-plugin-holochain-service, tauri-plugin-holochain-service-client and holochain-conductor-runtime-types-ffi to crates.io, triggered by release tag.
- The runtime network configuration must now be specified when initializing `tauri-plugin-holochain-service`. Previously, it was hard-coded.
- Upgrade to holochain 0.5
- Revert back to using p2p-shipyard nix flake

# 0.1.0
- Replace 3rd party `holochain_runtime` with `holochain-conductor-runtime`, modify FFI-bindings wrapper crate to use it. The runtime now makes calls via the `ConductorHandle` directly, without going through an `AdminWebsocket` to prevent unauthenticated access.
- Rename the FFI-bindings wrapper crate to `holochain-conductor-runtime-ffi` and generated kotlin bindings have been renamed accordingly.
- Removed the call `get_admin_port`, as an `AdminWebsocket` is no longer exposed.
- Split out types from holochain-conductor-runtime-ffi into separate crate holochain-conductor-runtime-types-ffi
- Create standalone kotlin library containing the types and client for binding and calling the HolochainService: `org.holochain.androidserviceruntime.holochain_service_client`
- Gradle setup for publishing standalone kotlin library to Maven Central
- Use published release in both tauri-plugin-holochain-service and tauri-plugin-holochain-service-consumer
- Minor fixes for android-service-runtime to get it working again
- Add function `isReady` to check that the conductor is available, expose in client and tauri-plugin-holochain-service
- Move uniffi-bindgen cli tool for generating bindings into its own crate, remove from *-ffi crates (see https://mozilla.github.io/uniffi-rs/0.27/tutorial/foreign_language_bindings.html#running-uniffi-bindgen-using-a-library-file-recommended)
- Commands and setup for publishing kotlin library to local maven repo. All other projects will check local maven repo *before* Maven Central.
- Android app logs warnings and errors to android system log.
- The following HolochainService functions exposed via IPC are now async and do not block the main thread: listApps, installApp, uninstallApp, enableApp, disableApp, isAppInstalled, ensureAppWebsocket, and signZomeCall.
- Run holochain-service-client tests in CI
- Fix Ffi types to Parcelable types converstions, and tests for Parcelable types.
- Support Json serialization of sealed classes, cleanup and tests of Json serialization.
- Added example app demonstrating use of tauri-plugin-holochain-service-consumer
- Fix inconsistent crashes on relaunch with "logger already initialized" errors.
- Add nix flake for android + holochain development, remove reliance on p2p-shipyard flake.
- Added example app demonstrating use of tauri-plugin-holochain-service-consumer
- Ensure the __HC_LAUNCHER_ENV__ is defined before the webview is initialized, by moving the app setup logic from injected JS in the webview to rust run during tauri setup.
- Add command `setupApp` to `HolochainServiceClient` that includes all logic for installing an app if necessary, enabling it, and setting up the app ws authentication.
- Remove commands `installApp`, `connect`, `isAppInstalled`, `ensureAppWebsocket` from `tauri-plugin-holochain-service-consumer`. Their function has been replaced by a single command `setupApp`.
- Display notice on consumer app launch when unable to connect to HolochainService.
- Refactor setupApp implementation by moving core logic into the `holochain-service-runtime` crate, rename `tauri-plugin-holochain-service-consumer` command from `setupApp` to `connectSetupApp`.
- Split HolochainService IPC binders into "admin" and "app" binders. Restrict admin binder calls to only the same android package as the service. Restrict app binder calls to only authorized package + happ pairs. Currently new package + happ pairs are authorized automatically.
- Extract HolochainService into standalone kotlin library `org.holochain.androidserviceruntime.holochain_service` to simplify testing. Run tests in CI.
- Re-added ktlint for kotlin linting
- Rename kotlin packages to resolve lint complaints
- Rename library and crate directories to reduce noise
- Android apps now require manual user approval to connect to a new holochain app
- Clearer npm package commands
- HolochainService will now auto start on system boot, if it was previously running during the last system shutdown.
- Fix bug in `apps/android-service-runtime`, where the app would crash if the service was not running when switching to the "My Apps" tab.
- Lint all kotlin libraries in CI
- Build android-service-runtime and example-client-app in CI
- Build html docs for both kotlin libraries, and publish to github pages, in CI.
