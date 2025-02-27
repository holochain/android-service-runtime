#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 \
  -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cargo run --bin uniffi-bindgen generate \
  --library ../../target/release/libholochain_runtime_types_uniffi.so \
  --out-dir ../../libraries/holochain-service-common/src/main/java/ \
  --language kotlin
