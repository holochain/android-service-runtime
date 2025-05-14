//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[HolochainService](index.md)/[onBind](on-bind.md)

# onBind

[androidJvm]\
open override fun [onBind](on-bind.md)(intent: [Intent](https://developer.android.com/reference/kotlin/android/content/Intent.html)): [IBinder](https://developer.android.com/reference/kotlin/android/os/IBinder.html)?

Called when a client binds to the service with bindService().

Returns either an AdminBinder or AppBinder based on the &quot;api&quot; extra in the intent. For AppBinder, also requires an &quot;installedAppId&quot; extra.

#### Return

An IBinder through which clients can call on to the service

#### Parameters

androidJvm

| | |
|---|---|
| intent | The Intent passed to bindService() |

#### Throws

| | |
|---|---|
| [InvalidParameterException](https://developer.android.com/reference/kotlin/java/security/InvalidParameterException.html) | if required parameters are missing or invalid |