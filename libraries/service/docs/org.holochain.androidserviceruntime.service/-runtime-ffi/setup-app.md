//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[RuntimeFfi](index.md)/[setupApp](setup-app.md)

# setupApp

[androidJvm]\
open suspend override fun [setupApp](setup-app.md)(payload: InstallAppPayloadFfi, enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): AppAuthFfi

Full process to setup an app

Check if app is installed, if not install it, then optionally enable it. Then ensure there is an app websocket and authentication for it.

If an app is already installed, it will not be enabled. It is only enabled after a successful install. The reasoning is that if an app is disabled after that point, it is assumed to have been manually disabled in the admin interface, which we don't want to override.