# holochain-conductor-runtime-ffi

This crate is a wrapper around `holochain-conductor-runtime`, with minimal functionality necessary to expose basic holochain management functions, for generating FFI bindings for those functions.

It generates FFI bindings for Kotlin.

## Building

- Run `./generate-bindings-release.sh`
- Copy the out/**/*.so directories into jniLib directory of your android project
- Copy the out/uniffi directory into java directory of your android project

## Development

### Gotchas!!!
- Enum variants cannot have the same name as other types (i.e. Error enum variants cannot match other error types)
- Generated types may have different casing. For example kotlin types use TitleCase