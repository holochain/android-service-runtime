# holochain_runtime_types_uniffi

This crate contains wrappers around types used in requests & responses of p2p-shipyard's `holochain_runtime`.

It generates FFI bindings for Kotlin.

## Building

- Run `./generate-bindings-release.sh`
- Copy the out/**/*.so directories into jniLib directory of your android project
- Copy the out/uniffi directory into java directory of your android project

## Development

### Gotchas!!!
- Enum variants cannot have the same name as other types (i.e. Error enum variants cannot match other error types)
- Generated types may have different casing. For example kotlin types use TitleCase