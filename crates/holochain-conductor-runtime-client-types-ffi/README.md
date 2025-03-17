# holochain-conductor-runtime-client-types-ffi

This crate contains types used by clients of `Runtime` from `holochain-conductor-runtime-ffi`.

It generates FFI bindings for Kotlin.

## Building

- Run `./scripts/build.sh`
- Bindings are output to the kotlin client library: `libraries/holochain-service-client`

## Gotchas
- Enum variants cannot have the same name as other types (i.e. Error enum variants cannot match other error types)
- Generated types may have different casing. For example kotlin types use TitleCase
- Error types cannot be used in external crates