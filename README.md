# Holochain Android Runtime

An Android app and Tauri plugins that provide an integrated approach for running Holochain apps on mobile devices. This includes:
- An Android app for managing a system-wide Holochain conductor running as an [Android Foreground Service](https://developer.android.com/develop/background-work/services/fgs). The Foreground Service can run persistently, even when the app is closed, ensuring that you can be a reliable contributor to the peer-to-peer networks of your apps.
- A Tauri plugin used by that Android app under-the-hood, for managing the system-wide Holochain conductor service.
- A Tauri plugin for your p2p-shipyard scaffolded app, so that it uses the system-wide Holochain conductor service, rather than bundling its own conductor.

> [!IMPORTANT]  
> This project currently uses a 3rd party library [p2p-shipyard](https://github.com/darksoil-studio/p2p-shipyard) under-the-hood, which is licensed as [Source-Available](https://en.wikipedia.org/wiki/Source-available_software). To use it in your own project, you will need a [p2p-shipyard license](https://darksoil.studio/p2p-shipyard/license/license.html).

## Components (from highest level to lowest)

### Android Apps

#### android-service-runtime

An Android app for managing a Holochain conductor running as a foreground service. Start & stop the Holochain service, view installed hApps, uninstall hApps.

Uses the [tauri-plugin-holochain-service](#tauri-plugin-holochain-service) under-the-hood to run a Holochain conductor as an android service.

### Tauri Plugins

#### tauri-plugin-holochain-service

A Tauri plugin for building Android apps that run a Holochain conductor as an [Foreground Service](https://developer.android.com/develop/background-work/services/fgs)

#### tauri-plugin-holochain-service-consumer

A Tauri plugin for building Android apps that make use of the android-service-runtime Android app, instead of bundling their own conductor.

### Kotlin Libraries

#### holochain-service-client

A Kotlin library containing a client class and types needed for connecting to the HolochainService in[tauri-plugin-holochain-service].

### Rust Crates

#### holochain-conductor-runtime

A slim wrapper around holochain Conductor with calls wrapping *some* AdminInterfaceApi requests. It currently only implements calls for the requests needed in this project.

#### holochain-conductor-runtime-ffi

A wrapper around [holochain-conductor-runtime](#holochain-conductor-runtime) with types compatible with Uniffi-generated FFI bindings, to facilitate usage of the crate in Kotlin code.

