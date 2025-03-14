# Holochain Service Client

A client library for calling the HolochainService which is exposed for IPC binding in the `android-service-runtime` Android app.

This library can be used by other projects that wish to use the holochain conductor bundled in the `android-service-runtime` app.

## How to Use in an Android App

Add the library to your dependencies list in your project's `build.gradle.kts`

```kotlin
dependencies {
    ...
    implementation("io.github.mattyg.holochain-service-client:holochain-service-client:0.0.1")
}
```

## Development

### Setup Publishing to Maven Central
1. Copy `gradle.properties.example` to `gradle.properties`
2. Create Maven Central Repository Account at https://central.sonatype.com
    1. Generate a User Token
    2. Set `mavenCentralUsername` and `mavenCentralPassword` in `gradle.properties` to the displayed username and password
3. Create signing key
    1. Create a key with: `gpg --gen-key`
    2. Export the private key to a file with: `gpg --keyring secring.gpg --export-secret-keys > ~/.gnupg/secring.gpg`
    3. Set `signing.password` in `gradle.properties` to the GPG key password
    4. Retrieve your public key id in short format with: `gpg --list-keys --keyid-format=short`, set it to `signing.keyId` in `gradle.properties`
    5. Publish your public key to keyserver with: `gpg --keyserver keys.openpgp.org --send-keys <YOUR KEY ID>`

### Publish to Maven Central

1. Update the version number in `build.gradle.kts`, under `mavenPublishing` > `coordinates`

2. Run the gradle command to publish: `gradlew publishAllPublicationsToMavenCentralRepository`