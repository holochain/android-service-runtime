#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a \
  -o ../../libraries/holochain-service-client/src/main/jniLibs/ \
  build

cargo run -p uniffi-bindgen --release generate \
  --library ../../target/aarch64-linux-android/debug/libholochain_conductor_runtime_types_ffi.so \
  --out-dir ../../libraries/holochain-service-client/src/main/java/ \
  --language kotlin
