# Holochain Service

A library containing the HolochainService.

This library can be used by other projects that wish to run a holochain conductor as an Android Foreground Service.

## How to Use in an Android App

Add the library to your dependencies list in your project's `build.gradle.kts`

```kotlin
dependencies {
    ...
    implementation("org.holochain.androidserviceruntime:holochain-service:0.0.1")
}
```

## Development

### Publish to local Maven repository

The library can be published to a local maven repository for use in development. All other kotlin projects in this android-service-runtime git repo will check for matching dependencies in the local Maven repository *before* checking Maven Central.

To publish the library to the local Maven repository, run:

```bash
pnpm run build:holochain-conductor-runtime-types-ffi && pnpm run publish:local:holochain-service
```

### Clear local Maven Repository

Clear the local Maven repository by deleting this library from it:

```bash
rm -rf ~/.m2/repository/org/holochain
```

### Run Tests

To run the test suite:

```bash
pnpm run build:holochain-conductor-runtime-types-ffi && pnpm run test:holochain-service
```

## Publishing

### Authorize Publishing to Maven Central
1. Copy `gradle.properties.example` to `gradle.properties`
2. Create Maven Central account at https://central.sonatype.com
    1. Generate a User Token
    2. Set `mavenCentralUsername` and `mavenCentralPassword` in `gradle.properties` to the displayed username and password
3. Ensure your Maven Central account has permissions to the `org.holochain` namespace (you'll need to ask someone else who has permission).
3. Create signing key
    1. Create a key with: `gpg --gen-key`
    2. Retrieve your Key Id in short format with: `gpg --list-keys --keyid-format=short`
    3. Publish your public key to keyserver with: `gpg --keyserver keys.openpgp.org --send-keys <YOUR KEY ID>`
4. Export the signing private key to a file with: `gpg --keyring secring.gpg --export-secret-keys > ~/.gnupg/secring.gpg`
5. Add signing key to `gradle.properties`
    1. Set `signing.keyId` to your public Key Id in in short format
    2. Set `signing.password` to the key password
    3. Set `signing.secretKeyRingFile` to the path to your exported private key file

### Publish to Maven Central

1. Update the version number in `build.gradle.kts`, under `mavenPublishing` > `coordinates`

2. Run the gradle command to publish: `./gradlew publishAllPublicationsToMavenCentralRepository`