# tauri-plugin-holochain-service-client

A Tauri plugin for holochain end-user apps.

With this plugin, you can write an android app that relies on holochain,
without bundling the holochain conductor.

Instead, this plugin expects that you have *another* android app installed,
with the tauri plugin `tauri-plugin-holochain-service`,
and will use that app's holochain conductor.

## Usage

See the [example-client-app](../../apps/example-client-app/src-tauri/src/lib.rs) for a example of how to use this plugin in your Tauri app.

## Internationalization

To add additional translation locales for android UI widgets, copy the directory `android/src/main/res/values` to `android/src/main/res/values-<LANGUAGE>[-<REGION>]`. Where `<LANGUAGE>` is replaced with the language code and `<REGION>` is replaced with the region code.
