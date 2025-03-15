package org.holochain.androidserviceruntime.holochain_service_client

import android.app.Activity
import android.content.ServiceConnection
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.os.IBinder
import android.util.Log

class HolochainServiceClient(
    private val activity: Activity,
    private val serviceClassName: String = "com.plugin.holochain_service.HolochainService",
    private val servicePackageName: String = "org.holochain.androidserviceruntime.app"
) {
    private var mService: IHolochainService? = null
    private val logTag = "HolochainServiceClient"

    // IPC Connection to HolochainService using AIDL
    private val mConnection = object : ServiceConnection {
        override fun onServiceConnected(className: ComponentName, service: IBinder) {
            mService = IHolochainService.Stub.asInterface(service)
            Log.d(logTag, "IHolochainService connected")
        }

        override fun onServiceDisconnected(className: ComponentName) {
            mService = null
            Log.d(logTag, "IHolochainService disconnected")
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
    fun listApps(): MutableList<AppInfoFfiParcel> {
        return this.mService!!.listApps()
    }

    /// Get or create an app websocket with authentication token
    fun ensureAppWebsocket(installedAppId: String): AppWebsocketFfiParcel {
        return this.mService!!.ensureAppWebsocket(installedAppId)
    }

    fun signZomeCall(args: ZomeCallUnsignedFfiParcel): ZomeCallFfiParcel {
        return this.mService!!.signZomeCall(args);
    }
}
