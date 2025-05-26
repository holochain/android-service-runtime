# holochain-conductor-runtime-types-ffi

This crate contains wrappers around types used in requests & responses of `holochain-conductor-runtime-ffi`, which are compatible with uniffi.

It generates FFI bindings for Kotlin.

## Build

This repo includes a script to build the crate as a library for android targets, generate kotlin bindings, and copy both the library and bindings into the kotlin library [`org.holochain.androidserviceruntime.client`](../../libraries/client/README.md).

Supported android targets are `aarch64-linux-android`, `x86_64-linux-android`, and `i686-linux-android`.

### For all supported android targets

To build for all supported android targets, run:

```bash
./build.sh
```

### For a single target

To build for a single android target, run:

```bash
./build-single-target.sh <TARGET-ARCH>
```

where `<TARGET_ARCH>` is the rust target triple, i.e. `aarch64-linux-android`.

## Uniffi Gotchas
- Enum variants cannot have the same name as other types (i.e. Error enum variants cannot match other error types)
- Generated types may have different casing. For example kotlin types use TitleCase