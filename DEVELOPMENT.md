# Contributing

To bump the holochain version you will need to do these things:
1. update all holochain dependencies in the top-level `Cargo.toml` file
2. do a `nix flake update` before `nix develop` to bring down the latest Holochain version.  Note that if you are updating to a major version you may need to change the source for `holonix.url` and `taurie-plugin-holochain.url` in the `flake.nix` file first.
3. make any updates necessary to the crates due to any changes in Holochain.  You should be able to check this by running `npm test` which should build all the crates and test the `client` and the `service` 
4. if there have been major changes you many need to create new fixtures for the tests in `crates/runtime/fixtures` and `crates/runtime-ffi/fixtures` by using the scaffolding tool from the current version of Holochain to build the `forum.happ` file (which you do by running `hc-scaffold example` and the following the instructions to initialize the generated repo and finally running `npm run package`)
5. Finally you to actually do a release you publish all the components listed in the following order:

## Publish runtime-types-ffi crate release

1. Bump crate version in `crates/runtime-types-ffi/Cargo.toml`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `runtime-types-ffi-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.
    
## Publish tauri-plugin-client crate release

1. Bump crate version in `crates/tauri-plugin-client/Cargo.toml`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `tauri-plugin-client-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.

## Publish tauri-plugin-service crate release

1. Bump crate version in `crates/tauri-plugin-service/Cargo.toml`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `tauri-plugin-service-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.

## Publish client library release

1. Bump versions of kotlin libraries.

    The are specified in `libraries/client/build.gradle.kts` under `mavenPublishing.coordinates`.

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `client-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.

## Publish service library release

1. Bump versions of kotlin libraries.

    The are specified in `libraries/service/build.gradle.kts` under `mavenPublishing.coordinates`.

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `service-vX.Y.Z`, where "X.Y.Z" is replaced with the version number.

## Publish app releases (android-service-runtime & example-client-app)

1. Bump versions of tauri apps.

    These are specified in multiple files (although only the `tauri.conf.json` file actually alters the built android app version):
    - `apps/*/src-tauri/tauri.conf.json`
    - `apps/*/src-tauri/Cargo.toml`
    - `apps/*/package.json`

2. Create and push a git tag with a matching format to trigger the release CI.

    The git tag format is: `android-service-runtime-vX.Y.Z`. Where "X.Y.Z" is replaced with the version number.

3. Wait for the release CI to complete. It will create a draft release, build both apps, and upload them to the draft release.
4. Visit the github releases page, and publish the draft release.




## CI

Note that the CI jobs `build-tauri-plugins` do *not* publish the kotlin client and service libraries to the local Maven repository.

If your PR makes changes to the client or service library, it will need to be published to Maven Central, before the CI jobs `build-tauri-plugins` will pass.
