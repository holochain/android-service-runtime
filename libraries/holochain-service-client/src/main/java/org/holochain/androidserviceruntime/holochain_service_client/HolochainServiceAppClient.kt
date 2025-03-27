package org.holochain.androidserviceruntime.holochain_service_client

import android.app.Activity
import android.content.ServiceConnection
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.os.IBinder
import android.util.Log
import kotlinx.coroutines.CompletableDeferred

class HolochainServiceAppClient(
    private val activity: Activity,
    private val installedAppId: String,
    private val serviceClassName: String = "com.plugin.holochain_service.HolochainService",
    private val servicePackageName: String = "org.holochain.androidserviceruntime.app"
) {
    private var mService: IHolochainServiceApp? = null
    private val TAG = "HolochainServiceAppClient"

    // IPC Connection to HolochainService using AIDL
    private val mConnection = object : ServiceConnection {
        override fun onServiceConnected(className: ComponentName, service: IBinder) {
            mService = IHolochainServiceApp.Stub.asInterface(service)
            Log.d(TAG, "onServiceConnected")
        }

        override fun onServiceDisconnected(className: ComponentName) {
            mService = null
            Log.d(TAG, "onServiceDisconnected")
        }
    }

    /// Start the service
    fun connect() {
        val intent = Intent()
        intent.setComponent(ComponentName(this.servicePackageName, this.serviceClassName))
        intent.putExtra("api", "app")
        intent.putExtra("installedAppId", this.installedAppId)
        
        this.activity.startForegroundService(intent)
        this.activity.bindService(intent, this.mConnection, Context.BIND_ABOVE_CLIENT)
    }

    /// Install an app
    suspend fun installApp(payload: InstallAppPayloadFfiParcel): AppInfoFfiParcel {
        Log.d(TAG, "installApp")

        val deferred = CompletableDeferred<AppInfoFfiParcel>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun installApp(response: AppInfoFfiParcel) {
                Log.d(TAG, "installApp callback")
                deferred.complete(response)
            }
        }
        this.mService!!.installApp(callbackBinder, payload)

        return deferred.await()
    }

    /// Is an app with the given app_id installed
    suspend fun isAppInstalled(): Boolean {
        Log.d(TAG, "isAppInstalled")

        val deferred = CompletableDeferred<Boolean>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun isAppInstalled(response: Boolean) {
                Log.d(TAG, "isAppInstalled callback")
                deferred.complete(response)
            }
        }
        this.mService!!.isAppInstalled(callbackBinder)

        return deferred.await()
    }

    /// Enable an installed app
    suspend fun enableApp(): AppInfoFfiParcel {
        Log.d(TAG, "enableApp")
        val deferred = CompletableDeferred<AppInfoFfiParcel>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun enableApp(response: AppInfoFfiParcel) {
                Log.d(TAG, "enableApp callback")
                deferred.complete(response)
            }
        }
        this.mService!!.enableApp(callbackBinder)
        return deferred.await()
    }

    /// Get or create an app websocket with authentication token
    suspend fun ensureAppWebsocket(): AppAuthFfiParcel {
        Log.d(TAG, "ensureAppWebsocket")

        val deferred = CompletableDeferred<AppAuthFfiParcel>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun ensureAppWebsocket(response: AppAuthFfiParcel) {
                Log.d(TAG, "ensureAppWebsocket callback")
                deferred.complete(response)
            }
        }
        this.mService!!.ensureAppWebsocket(callbackBinder)

        return deferred.await()
    }

    /// Sign a zome call
    suspend fun signZomeCall(args: ZomeCallUnsignedFfiParcel): ZomeCallFfiParcel {
        Log.d(TAG, "signZomeCall")

        val deferred = CompletableDeferred<ZomeCallFfiParcel>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun signZomeCall(response: ZomeCallFfiParcel) {
                Log.d(TAG, "signZomeCall callback")
                deferred.complete(response)
            }
        }
        this.mService!!.signZomeCall(callbackBinder, args)

        return deferred.await()
    }
}
