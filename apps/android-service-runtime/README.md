# android-service-runtime app

An Android app for managing a system-wide Holochain conductor running as an [Android Foreground Service](https://developer.android.com/develop/background-work/services/fgs).

The Foreground Service can run persistently, even when the app is closed, ensuring that you can be a reliable contributor to the peer-to-peer networks of your apps.

## Build

- Copy `src-tauri/gen/android/key.properties.example` to `src-tauri/gen/android/key.properties` and fill in the values

### Build as a Standalone User App

By default, this will be built as a standard user app that only appears in the App Grid. 

### Build as a System App

To build as a system app that only appears in the Android System Settings page, use the feature flag `system_settings`.

i.e. `npm run tauri build -- --features system_settings`

## Configure

The network settings of the holochain runtime are configurable via the app UI.

Most of the network settings must be set by the distributor of this app, rather than the users, to ensure all users can communicate with each other.

To modify the network config, edit the [config file](./config).
