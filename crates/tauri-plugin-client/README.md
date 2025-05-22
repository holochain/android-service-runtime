# tauri-plugin-holochain-service-client

Tauri Plugin to interact with the HolochainService as a client app -- i.e. without bundling the holochain conductor, but expecting another app to have already installed and run the foreground service.

## Internationalization

To add additional translation locales for android UI widgets, copy the directory `android/src/main/res/values` to `android/src/main/res/values-<LANGUAGE>[-<REGION>]`. Where `<LANGUAGE>` is replaced with the language code and `<REGION>` is replaced with the region code.
