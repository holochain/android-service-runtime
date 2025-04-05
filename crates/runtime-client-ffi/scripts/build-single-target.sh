#/usr/bin/env bash

TARGET=$1

cargo ndk --manifest-path ./Cargo.toml -t $TARGET \
  -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

cargo run -p uniffi-bindgen-cli --release generate \
  --library ../../target/$TARGET/release/libholochain_conductor_runtime_client_ffi.so \
  --out-dir ../../libraries/holochain-service-client/src/main/java/ \
  --language kotlin
