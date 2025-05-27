# Contributing

## Publishing app releases

1. Bump versions of tauri apps.

    These are specified in multiple files (although only the `tauri.conf.json` file actually alters the built android app version):
    - `apps/*/src-tauri/tauri.conf.json`
    - `apps/*/src-tauri/Cargo.toml`
    - `apps/*/package.json`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `android-service-runtime-vX.Y.Z`. Where "X.Y.Z" is replaced with the version number.

3. Wait for the release CI to complete. It will create a draft release, build both apps, and upload them to the draft release.
4. Visit the github releases page, and publish the draft release.


## Publishing client library releases

1. Bump versions of kotlin libraries.

    The are specified in `libraries/client/build.gradle.kts` under `mavenPublishing.coordinates`.

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `client-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.

## Publishing service library releases

1. Bump versions of kotlin libraries.

    The are specified in `libraries/service/build.gradle.kts` under `mavenPublishing.coordinates`.

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `service-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.


## Publishing runtime-types-ffi crate releases

1. Bump crate version in `crates/runtime-types-ffi/Cargo.toml`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `runtime-types-ffi-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.
    
## Publishing tauri-plugin-client crate releases

1. Bump crate version in `crates/tauri-plugin-client/Cargo.toml`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `tauri-plugin-client-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.

    
## Publishing tauri-plugin-service crate releases

1. Bump crate version in `crates/tauri-plugin-service/Cargo.toml`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `tauri-plugin-service-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.