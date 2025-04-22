#/usr/bin/env bash

TARGET=$1

cargo build -p holochain-conductor-runtime-types-ffi --release

cargo ndk --manifest-path ./Cargo.toml -t $TARGET \
  -o ../../libraries/client/src/main/jniLibs \
  build --release

cargo run -p uniffi-bindgen-cli --release generate \
  --library ../../target/$TARGET/release/libholochain_conductor_runtime_types_ffi.so \
  --out-dir ../../libraries/client/src/main/java/ \
  --language kotlin
