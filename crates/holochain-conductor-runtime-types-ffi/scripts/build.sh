#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 \
  -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cargo run --bin uniffi-bindgen generate \
  --library ../../target/aarch64-linux-android/release/libholochain_conductor_runtime_types_ffi.so \
  --out-dir ../../libraries/holochain-service-types/src/main/java/ \
  --language kotlin
