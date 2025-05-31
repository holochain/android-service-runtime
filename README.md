# Holochain Android Runtime

An Android app and Tauri plugins that provide an integrated approach for running Holochain apps on mobile devices. This includes:
- An Android app for managing a system-wide Holochain conductor running as an [Android Foreground Service](https://developer.android.com/develop/background-work/services/fgs). The Foreground Service can run persistently, even when the app is closed, ensuring that you can be a reliable contributor to the peer-to-peer networks of your apps.
- A Tauri plugin used by that Android app under-the-hood, for managing the system-wide Holochain conductor service.
- A Tauri plugin for your tauri holochain app, so that it uses the system-wide Holochain conductor service, rather than bundling its own conductor.


## Components

### Android Apps

#### android-service-runtime

An Android app for managing a Holochain conductor running as a foreground service. Start & stop the Holochain service, view installed hApps, uninstall hApps.

Uses the [tauri-plugin-holochain-service](#tauri-plugin-holochain-service) under-the-hood to run a Holochain conductor as an android service.

[See README](./apps/android-service-runtime/README.md)


#### example-client-app

An example holochain app, based on the scaffolded example forum app.

Uses the [tauri-plugin-holochain-service-client](#tauri-plugin-holochain-service) under-the-hood to connect to the Holochain conductor provided by the [android-service-runtime](#android-service-runtime).

[See README](./apps/android-service-runtime/README.md)


### Tauri Plugins

#### tauri-plugin-holochain-service

A Tauri plugin for building Android apps that run a Holochain conductor as an [Foreground Service](https://developer.android.com/develop/background-work/services/fgs)

[See README](./crates/tauri-plugin-service/README.md)

#### tauri-plugin-holochain-service-client

A Tauri plugin for building Android apps that make use of the android-service-runtime Android app, instead of bundling their own conductor.

[See README](./crates/tauri-plugin-client/README.md)

### Kotlin Libraries

#### org.holochain.androidserviceruntime.client

A Kotlin library containing a client class and types needed for connecting to the HolochainService in [holochain-service].

[See README](./libraries/client/README.md)

##### Documentation

[HolochainServiceAppClient](libraries/client/docs/org.holochain.androidserviceruntime.client/-holochain-service-app-client/index.md)

[HolochainServiceAdminClient](libraries/client/docs/org.holochain.androidserviceruntime.client/-holochain-service-admin-client/index.md)

#### org.holochain.androidserviceruntime.service

A Kotlin library containing the HolochainService class, which runs an Android Foreground Service that wraps calls to the [holochain-conductor-runtime-ffi] and exposes an IPC interface for interacting with it.

[Documentation](libraries/client/docs/org.holochain.androidserviceruntime.service/index.md)

[See README](./libraries/service/README.md)

### Rust Crates

#### holochain-conductor-runtime

A slim wrapper around holochain Conductor with calls wrapping *some* AdminInterfaceApi requests. It currently only implements calls for the requests needed in this project.

[See README](./crates/runtime/README.md)

#### holochain-conductor-runtime-ffi

A wrapper around [holochain-conductor-runtime](#holochain-conductor-runtime) with types from [holochain-conductor-runtime-types-ffi](#holochain-conductor-runtime-types-ffi) with Uniffi-generated FFI bindings, to facilitate usage of the crate in Kotlin code.

[See README](./crates/runtime-ffi/README.md)

#### holochain-conductor-runtime-types-ffi

The input and output types used in [holochain-conductor-runtime-ffi](#holochain-conductor-runtime), compatible with Uniffi-generated FFI bindings, to facilitate usage of the crate in Kotlin code.

The FFI types are defined in a separate crate to ensure they can be used in the kotlin client library, without needing to bundle the entire runtime.

[See README](./crates/runtime-types-ffi/README.md)


## Development

See the `README.md` files within each component for development info.

See the [DEVELOPMENT.md](./DEVELOPMENT.md) for additional developer information, not specific to one component.