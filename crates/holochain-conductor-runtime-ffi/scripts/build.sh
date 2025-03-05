#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 \
  -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cp ../../target/release/libholochain_runtime_uniffi.so ../tauri-plugin-holochain-service/android/src/main/jniLibs

cargo run --bin uniffi-bindgen generate \
  --library ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  --language kotlin \
  --out-dir ../tauri-plugin-holochain-service/android/src/main/java/
