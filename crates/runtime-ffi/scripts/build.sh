#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 \
 -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cargo run -p uniffi-bindgen-cli --release generate \
  --library ../../target/aarch64-linux-android/release/libholochain_conductor_runtime_ffi.so \
  --crate holochain_conductor_runtime_ffi \
  --out-dir ../tauri-plugin-holochain-service/android/src/main/java/ \
  --language kotlin