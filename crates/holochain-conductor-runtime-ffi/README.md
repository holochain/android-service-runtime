# holochain-conductor-runtime-ffi

This crate is a wrapper around `holochain-conductor-runtime`, with minimal functionality necessary to expose basic holochain conductor and lair admin functions, compatible with [uniffi](https://github.com/mozilla/uniffi-rs)

It generates FFI bindings for Kotlin.

## Building

- Run `./scripts/build.sh`
- Bindings are output to the tauri-plugin-holochain-service: `crates/tauri-plugin-holochain-service`

## Gotchas
- Enum variants cannot have the same name as other types (i.e. Error enum variants cannot match other error types)
- Generated types may have different casing. For example kotlin types use TitleCase
- Error types cannot be used in external crates