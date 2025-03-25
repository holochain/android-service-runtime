package org.holochain.androidserviceruntime.holochain_service_client

import android.app.Activity
import android.content.ServiceConnection
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.os.IBinder
import android.util.Log
import kotlinx.coroutines.delay

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
    fun installApp(payload: InstallAppPayloadFfiParcel): AppInfoFfiParcel {
        // Call installApp on service
        return this.mService!!.installApp(payload)
    }

    /// Is an app with the given app_id installed
    fun isAppInstalled(installedAppId: String): Boolean {
        return this.mService!!.isAppInstalled(installedAppId)
    }

    /// Uninstall an installed app
    fun uninstallApp(installedAppId: String) {
        this.mService!!.uninstallApp(installedAppId)
    }

    /// Enable an installed app
    fun enableApp(installedAppId: String) {
        this.mService!!.enableApp(installedAppId)
    }

    /// Disable an installed app
    fun disableApp(installedAppId: String) {
        this.mService!!.disableApp(installedAppId)
    }

    /// List installed happs in conductor
    suspend fun listApps(): List<AppInfoFfiParcel> {
        Log.d(TAG, "listApps")

        var res: List<AppInfoFfiParcel>? = null
        val callbackBinder = object: IHolochainServiceCallback.Stub() {
            override fun listApps(response: List<AppInfoFfiParcel>) {
                Log.d(TAG, "listApps Callback")
                res = response
            }
        }
        this.mService!!.listApps(callbackBinder)

        while(res == null) {
            delay(50)
        }
        return res
    }

    /// Get or create an app websocket with authentication token
    fun ensureAppWebsocket(installedAppId: String): AppAuthFfiParcel {
        return this.mService!!.ensureAppWebsocket(installedAppId)
    }

    fun signZomeCall(args: ZomeCallUnsignedFfiParcel): ZomeCallFfiParcel {
        return this.mService!!.signZomeCall(args);
    }
}
