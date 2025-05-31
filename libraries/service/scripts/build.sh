#!/usr/bin/env bash

# Copy the dependency lib++_shared.so into the jniLibs that will be bundled with our library
#
# This is a dependency of the holochain-conductor-runtime crate, because of holochain's dependency on a c++ library libdatachannel.

cp $ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so src/main/jniLibs/arm64-v8a/
cp $ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/x86_64-linux-android/libc++_shared.so src/main/jniLibs/x86_64/
cp $ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/i386-linux-android/libc++_shared.so src/main/jniLibs/x86/

# Run gradle build
./gradlew build