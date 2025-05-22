#/usr/bin/env bash

cargo ndk --manifest-path ./Cargo.toml -t arm64-v8a -t x86 -t x86_64 build