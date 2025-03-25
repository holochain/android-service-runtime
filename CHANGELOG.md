# Unreleased

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
