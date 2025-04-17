#/usr/bin/env bash

cargo build -p holochain-conductor-runtime-ffi

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 \
 -o ../../libraries/holochain-service/src/main/jniLibs \
  build --release

# Delete the types-ffi library from tauri-plugin-holochain-service, as it will be included via the dependency on holochain-service-client kotlin library
find "../../libraries/holochain-service/src/main/jniLibs/" -wholename '**/libholochain_conductor_runtime_types_ffi.so' -delete

cargo run -p uniffi-bindgen-cli --release generate \
  --library ../../target/aarch64-linux-android/release/libholochain_conductor_runtime_ffi.so \
  --crate holochain_conductor_runtime_ffi \
  --out-dir ../../libraries/holochain-service/src/main/java/ \
  --language kotlin