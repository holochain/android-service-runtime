#/usr/bin/env bash

TARGET=$1

cargo ndk --manifest-path ./Cargo.toml -t $TARGET \
 -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cargo run -p uniffi-bindgen --release generate \
  --library ../../target/$TARGET/release/libholochain_conductor_runtime_ffi.so \
  --crate holochain_conductor_runtime_ffi \
  --out-dir ../tauri-plugin-holochain-service/android/src/main/java/ \
  --language kotlin