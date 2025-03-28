# holochain_runtime_types_uniffi

This crate contains wrappers around types used in requests & responses of p2p-shipyard's `holochain_runtime`.

It generates FFI bindings for Kotlin.

## Building

- Run `./generate-bindings-release.sh`
- Copy the out/**/*.so directories into jniLib directory of your android project
- Copy the out/uniffi directory into java directory of your android project

## Development

### Adding Types
When adding new types to this crate that will be used in FFI-exposed interfaces in `holochain_runtime_uniffi` crate, you will also need to add the type to the [UDL file](../holochain_runtime_uniffi/src/holochain_runtime_uniffi.udl) in `holochain_runtime_uniffi`.

See [Uniffi Docs](https://mozilla.github.io/uniffi-rs/latest/udl/external_types.html) for more info on including external types.

### Gotchas!!!
- Enum variants cannot have the same name as other types (i.e. Error enum variants cannot match other error types)
- Generated types may have different casing. For example kotlin types use TitleCase