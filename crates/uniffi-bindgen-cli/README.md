# uniffi-bindgen-cli

This crate provides a binary of the uniffi-bindgen cli, used for generating ffi bindings.

It is used in the build scripts of [`holochain-conductor-runtime-ffi`](../runtime-ffi/README.md) and [`holochain-conductor-runtime-types-ffi`](../runtime-types-ffi/README.md).

Uniffi does not provide this binary, but expects consumer crates to make their own.

See [the uniffi docs](https://mozilla.github.io/uniffi-rs/0.27/tutorial/foreign_language_bindings.html#running-uniffi-bindgen-using-a-library-file-recommended)