# android-service-runtime app

An Android app for managing a system-wide Holochain conductor running as an [Android Foreground Service](https://developer.android.com/develop/background-work/services/fgs).

The Foreground Service can run persistently, even when the app is closed, ensuring that your node remains a participant in the peer-to-peer networks of your apps.

## Build

- Copy `src-tauri/gen/android/key.properties.example` to `src-tauri/gen/android/key.properties` and fill in the values

### Build as a Standalone User App

By default, this will be built as a standard user app that only appears in the App Grid. 

### Build as a System App

To build as a system app that only appears in the Android System Settings page, use the feature flag `system_settings`.

i.e. `npm run tauri build -- --features system_settings`

## Development

### Internationalization

To add additional translation locales to the android-service-runtime webview UI, fill in the corresponding file `src/translations/locales/<LANGUAGE>.json` where `<LANGUAGE>` is the language code. Copy the string keys from `src/translations/locales/en.json` which is the default fallback language.


To add additional translation locales to the android native UI widgets, see the [tauri-plugin-holochain-service-client docs](../../crates/tauri-plugin-client/README.md#internationalization).
