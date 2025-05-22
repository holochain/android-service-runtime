#/usr/bin/env bash

TARGET=$1

cargo ndk --manifest-path ./Cargo.toml -t $TARGET  build