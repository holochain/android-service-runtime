#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a \
 -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build

cargo run -p uniffi-bindgen --release generate \
  --library ../../target/aarch64-linux-android/debug/libholochain_conductor_runtime_ffi.so \
  --crate holochain_conductor_runtime_ffi \
  --out-dir ../tauri-plugin-holochain-service/android/src/main/java/ \
  --language kotlin