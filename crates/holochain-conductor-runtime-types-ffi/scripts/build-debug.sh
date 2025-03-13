#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a \
  -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build

cargo run --bin uniffi-bindgen generate \
  --library ../../target/aarch64-linux-android/debug/libholochain_conductor_runtime_types_ffi.so \
  --out-dir ../../libraries/holochain-service-types/src/main/java/ \
  --language kotlin
