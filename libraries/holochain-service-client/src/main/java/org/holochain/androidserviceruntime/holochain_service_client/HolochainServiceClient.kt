package org.holochain.androidserviceruntime.holochain_service_client

import android.app.Activity
import android.content.ServiceConnection
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.os.IBinder
import android.util.Log
import kotlinx.coroutines.CompletableDeferred

class HolochainServiceClient(
    private val activity: Activity,
    private val serviceClassName: String = "com.plugin.holochain_service.HolochainService",
    private val servicePackageName: String = "org.holochain.androidserviceruntime.app"
) {
    private var mService: IHolochainService? = null
    private val TAG = "HolochainServiceClient"

    // IPC Connection to HolochainService using AIDL
    private val mConnection = object : ServiceConnection {
        override fun onServiceConnected(className: ComponentName, service: IBinder) {
            mService = IHolochainService.Stub.asInterface(service)
            Log.d(TAG, "IHolochainService connected")
        }

        override fun onServiceDisconnected(className: ComponentName) {
            mService = null
            Log.d(TAG, "IHolochainService disconnected")
        }
    }

    /// Start the service
    fun connect() {
        val intent = Intent()
        intent.setComponent(ComponentName(this.servicePackageName, this.serviceClassName))

        this.activity.startForegroundService(intent)
        this.activity.bindService(intent, this.mConnection, Context.BIND_ABOVE_CLIENT)
    }
    
    /// Stop the service
    fun stop() {
        this.mService!!.stop()
    }
        
    /// Is the service ready to receive calls
    fun isReady(): Boolean {
        return this.mService!!.isReady()
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
    fun isAppInstalled(installedAppId: String): Boolean {
        return this.mService!!.isAppInstalled(installedAppId)
    }

    /// Uninstall an installed app
    suspend fun uninstallApp(installedAppId: String) {
        Log.d(TAG, "uninstallApp")

        val deferred = CompletableDeferred<Unit>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun uninstallApp() {
                Log.d(TAG, "uninstallApp callback")
                deferred.complete(Unit)
            }
        }
        this.mService!!.uninstallApp(callbackBinder, installedAppId)

        deferred.await()
    }

    /// Enable an installed app
    suspend fun enableApp(installedAppId: String): AppInfoFfiParcel {
        val deferred = CompletableDeferred<AppInfoFfiParcel>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun enableApp(response: AppInfoFfiParcel) {
                Log.d(TAG, "enableApp callback")
                deferred.complete(response)
            }
        }
        this.mService!!.enableApp(callbackBinder, installedAppId)
        return deferred.await()
    }

    /// Disable an installed app
    fun disableApp(installedAppId: String) {
        this.mService!!.disableApp(installedAppId)
    }

    /// List installed happs in conductor
    suspend fun listApps(): List<AppInfoFfiParcel> {
        Log.d(TAG, "listApps")

        val deferred = CompletableDeferred<List<AppInfoFfiParcel>>()
        var callbackBinder = object : IHolochainServiceCallbackStub() {
            override fun listApps(response: List<AppInfoFfiParcel>) {
                Log.d(TAG, "listApps callback")
                deferred.complete(response)
            }
        }
        this.mService!!.listApps(callbackBinder)

        return deferred.await()
    }

    /// Get or create an app websocket with authentication token
    fun ensureAppWebsocket(installedAppId: String): AppAuthFfiParcel {
        return this.mService!!.ensureAppWebsocket(installedAppId)
    }

    fun signZomeCall(args: ZomeCallUnsignedFfiParcel): ZomeCallFfiParcel {
        return this.mService!!.signZomeCall(args);
    }
}
