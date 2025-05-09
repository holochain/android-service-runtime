//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[BootCompletedReceiver](index.md)/[onReceive](on-receive.md)

# onReceive

[androidJvm]\
open override fun [onReceive](on-receive.md)(context: [Context](https://developer.android.com/reference/kotlin/android/content/Context.html), intent: [Intent](https://developer.android.com/reference/kotlin/android/content/Intent.html))

Called when the device has completed booting.

This method checks for the ACTION_BOOT_COMPLETED intent and starts the HolochainService with the ACTION_BOOT_COMPLETED action. The service will then decide whether to start Holochain based on the user's autostart configuration.

#### Parameters

androidJvm

| | |
|---|---|
| context | The Context in which the receiver is running |
| intent | The Intent being received |