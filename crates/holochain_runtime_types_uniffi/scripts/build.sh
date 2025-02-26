#/usr/bin/env bash

cargo build --release

cargo ndk -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  --manifest-path ./Cargo.toml \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86 \
  -t x86_64 \
  build \
  --release

cargo run --bin uniffi-bindgen generate \
  --library ../../target/release/libholochain_runtime_types_uniffi.so \
  --language kotlin \
  --out-dir ../tauri-plugin-holochain-service/android/src/main/java/
