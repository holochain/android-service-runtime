#/usr/bin/env bash

TARGET=$1

cargo build -p holochain-conductor-runtime-ffi

cargo ndk --manifest-path ./Cargo.toml -t $TARGET \
 -o ../tauri-plugin-holochain-service/android/src/main/jniLibs \
  build --release

# Delete the types-ffi library from tauri-plugin-holochain-service, as it will be included via the dependency on holochain-service-client kotlin library
find "../tauri-plugin-holochain-service/android/src/main/jniLibs/" -wholename '**/libholochain_conductor_runtime_types_ffi.so' -delete

cargo run -p uniffi-bindgen-cli --release generate \
  --library ../../target/$TARGET/release/libholochain_conductor_runtime_ffi.so \
  --crate holochain_conductor_runtime_ffi \
  --out-dir ../tauri-plugin-holochain-service/android/src/main/java/ \
  --language kotlin