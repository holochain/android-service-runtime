package org.holochain.androidserviceruntime.holochain_service_client

import android.app.Activity
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.util.Log
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.delay

class HolochainServiceAdminClient(
    private val activity: Activity,
    private val servicePackageName: String = "org.holochain.androidserviceruntime.app",
    private val serviceClassName: String = "com.plugin.holochain_service.HolochainService",
) {
  private var mService: IHolochainServiceAdmin? = null
  private val TAG = "HolochainServiceAdminClient"

  // IPC Connection to HolochainService using AIDL
  private val mConnection =
      object : ServiceConnection {
        override fun onServiceConnected(className: ComponentName, service: IBinder) {
          mService = IHolochainServiceAdmin.Stub.asInterface(service)
          Log.d(TAG, "IHolochainServiceAdmin connected")
        }

        override fun onServiceDisconnected(className: ComponentName) {
          mService = null
          Log.d(TAG, "IHolochainServiceAdmin disconnected")
        }
      }

  /// Entire process to setup an app
  ///
  /// Connect to service, install app, enable app, ensure app websocket
  suspend fun setupApp(
      installAppPayload: InstallAppPayloadFfi,
      enableAfterInstall: Boolean
  ): AppAuthFfi {
    Log.d(TAG, "setupApp")
    this.connect()
    this.waitForConnectReady()

    if (!this.isAppInstalled(installAppPayload.installedAppId!!)) {
      this.installApp(installAppPayload)

      if (enableAfterInstall) {
        this.enableApp(installAppPayload.installedAppId!!)
      }
    }

    return this.ensureAppWebsocket(installAppPayload.installedAppId!!)
  }

  /// Connect to the service
  fun connect() {
    Log.d(TAG, "connect")
    val intent = Intent()
    intent.putExtra("api", "admin")
    intent.setComponent(ComponentName(this.servicePackageName, this.serviceClassName))
    this.activity.bindService(intent, this.mConnection, Context.BIND_ABOVE_CLIENT)
  }

  /// Poll until we are connected to the service, or the timeout has elapsed
  suspend fun waitForConnectReady(timeoutMs: Long = 100L, intervalMs: Long = 5L) {
    var elapsedMs = 0L
    while (elapsedMs <= timeoutMs) {
      Log.d(TAG, "waitForConnectReady " + elapsedMs)
      if (this.mService != null) break

      delay(intervalMs)
      elapsedMs += intervalMs
    }
  }

  /// Stop the service
  fun stop() {
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }
    this.mService!!.stop()
  }

  /// Is the service ready to receive calls
  fun isReady(): Boolean {
    if (this.mService == null) {
      return false
    }
    return this.mService!!.isReady()
  }

  /// Install an app
  suspend fun installApp(payload: InstallAppPayloadFfi): AppInfoFfi {
    Log.d(TAG, "installApp")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }

    val deferred = CompletableDeferred<AppInfoFfi>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun installApp(response: AppInfoFfiParcel) {
            Log.d(TAG, "installApp callback")
            deferred.complete(response.inner)
          }
        }
    this.mService!!.installApp(callbackBinder, InstallAppPayloadFfiParcel(payload))

    return deferred.await()
  }

  /// Is an app with the given app_id installed
  suspend fun isAppInstalled(installedAppId: String): Boolean {
    Log.d(TAG, "isAppInstalled")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }

    val deferred = CompletableDeferred<Boolean>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun isAppInstalled(response: Boolean) {
            Log.d(TAG, "isAppInstalled callback")
            deferred.complete(response)
          }
        }
    this.mService!!.isAppInstalled(callbackBinder, installedAppId)

    return deferred.await()
  }

  /// Uninstall an installed app
  suspend fun uninstallApp(installedAppId: String) {
    Log.d(TAG, "uninstallApp")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }

    val deferred = CompletableDeferred<Unit>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun uninstallApp() {
            Log.d(TAG, "uninstallApp callback")
            deferred.complete(Unit)
          }
        }
    this.mService!!.uninstallApp(callbackBinder, installedAppId)

    deferred.await()
  }

  /// Enable an installed app
  suspend fun enableApp(installedAppId: String): AppInfoFfi {
    Log.d(TAG, "enableApp")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }
    
    val deferred = CompletableDeferred<AppInfoFfi>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun enableApp(response: AppInfoFfiParcel) {
            Log.d(TAG, "enableApp callback")
            deferred.complete(response.inner)
          }
        }
    this.mService!!.enableApp(callbackBinder, installedAppId)
    return deferred.await()
  }

  /// Disable an installed app
  suspend fun disableApp(installedAppId: String) {
    Log.d(TAG, "disableApp")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }
    
    val deferred = CompletableDeferred<Unit>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun disableApp() {
            Log.d(TAG, "disableApp callback")
            deferred.complete(Unit)
          }
        }
    this.mService!!.disableApp(callbackBinder, installedAppId)
    deferred.await()
  }

  /// List installed happs in conductor
  suspend fun listApps(): List<AppInfoFfi> {
    Log.d(TAG, "listApps")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }

    val deferred = CompletableDeferred<List<AppInfoFfi>>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun listApps(response: List<AppInfoFfiParcel>) {
            Log.d(TAG, "listApps callback")
            deferred.complete(response.map { it.inner })
          }
        }
    this.mService!!.listApps(callbackBinder)

    return deferred.await()
  }

  /// Get or create an app websocket with authentication token
  suspend fun ensureAppWebsocket(installedAppId: String): AppAuthFfi {
    Log.d(TAG, "ensureAppWebsocket")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }

    val deferred = CompletableDeferred<AppAuthFfi>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun ensureAppWebsocket(response: AppAuthFfiParcel) {
            Log.d(TAG, "ensureAppWebsocket callback")
            deferred.complete(response.inner)
          }
        }
    this.mService!!.ensureAppWebsocket(callbackBinder, installedAppId)

    return deferred.await()
  }

  /// Sign a zome call
  suspend fun signZomeCall(args: ZomeCallUnsignedFfi): ZomeCallFfi {
    Log.d(TAG, "signZomeCall")
    if (this.mService == null) {
      throw HolochainServiceNotConnectedException()
    }

    val deferred = CompletableDeferred<ZomeCallFfi>()
    var callbackBinder =
        object : IHolochainServiceCallbackStub() {
          override fun signZomeCall(response: ZomeCallFfiParcel) {
            Log.d(TAG, "signZomeCall callback")
            deferred.complete(response.inner)
          }
        }
    this.mService!!.signZomeCall(callbackBinder, ZomeCallUnsignedFfiParcel(args))

    return deferred.await()
  }
}
