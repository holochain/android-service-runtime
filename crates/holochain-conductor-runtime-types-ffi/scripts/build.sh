#/usr/bin/env bash

cargo build -p holochain-conductor-runtime-types-ffi

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t x86 -t x86_64 \
  -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cargo run -p uniffi-bindgen-cli --release generate \
  --library ../../target/aarch64-linux-android/release/libholochain_conductor_runtime_types_ffi.so \
  --out-dir ../../libraries/holochain-service-client/src/main/java/ \
  --language kotlin
