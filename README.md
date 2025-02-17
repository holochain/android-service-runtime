#  android-service-runtime

An Android app for managing a holochain conductor running as a foreground service.

## Build

### Build as a Standalone User App

By default, this will be built as a standard user app that only appears in the App Grid. 

### Build as a System App

To build as a system app that only appears in the Android System Settings page, use the feature flag `system_settings`.

i.e. `npm run tauri build -- --features system_settings`
