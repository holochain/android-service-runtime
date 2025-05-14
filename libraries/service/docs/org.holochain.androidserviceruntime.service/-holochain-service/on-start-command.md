//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[HolochainService](index.md)/[onStartCommand](on-start-command.md)

# onStartCommand

[androidJvm]\
open override fun [onStartCommand](on-start-command.md)(intent: [Intent](https://developer.android.com/reference/kotlin/android/content/Intent.html)?, flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html), startId: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)): [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)

Called by the system when the service is first created or when an intent is delivered to the service.

Handles several actions:

- 
   ACTION_START: Enables autostart and starts the Holochain service
- 
   ACTION_BOOT_COMPLETED: Starts the service if autostart is enabled
- 
   ACTION_APPROVE_APP_AUTHORIZATION: Authorizes an app to access Holochain
- 
   ACTION_DENY_APP_AUTHORIZATION: Dismisses an authorization request

#### Return

How to continue running the service (NOT_STICKY in this case)

#### Parameters

androidJvm

| | |
|---|---|
| intent | The Intent supplied to startService(Intent) |
| flags | Additional data about this start request |
| startId | A unique integer representing this specific request to start |