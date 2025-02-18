package com.plugin.holochain_service

import android.app.Activity
import android.content.Intent
import android.content.ServiceConnection
import android.content.ComponentName
import android.os.IBinder
import android.util.Log
import android.os.SharedMemory
import java.nio.ByteBuffer

class HolochainServiceClient(private val activity: Activity) {
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
    /// - Starts the foreground service
    /// - Launches a conductor
    /// - Creates an admin websocket
    fun launch() {
        // Start service
        val intent = Intent(activity, HolochainService::class.java)
        activity.startForegroundService(intent)
        activity.bindService(intent, this.mConnection, 0)
    }
    
    /// Stop the service
    fun shutdown() {
        this.mService!!.shutdown()
    }

    /// Get the holochain conductor admin websocket port
    fun getAdminPort(): Int {
        return this.mService!!.getAdminPort()
    }

    /// Install a happ into conductor
    fun installApp(args: InstallAppRequestArgs) {
        // Write appBundleBytes to shared memory
        val appBundleSharedMemory = SharedMemory.create(args.appId, args.appBundleBytes.size)
        val appBundleSharedMemoryBuffer: ByteBuffer = appBundleSharedMemory.mapReadWrite()
        appBundleSharedMemoryBuffer.put(args.appBundleBytes)

        // Call installApp on service
        this.mService!!.installApp(InstallAppRequestAidl(
            args.appId,
            appBundleSharedMemory,
            args.membraneProofs,
            args.agent,
            args.networkSeed
        ))

        // Clear the shared memory
        SharedMemory.unmap(appBundleSharedMemoryBuffer)
        appBundleSharedMemory.close()
    }

    /// Is an app with the given app_id installed
    fun isAppInstalled(appId: String): Boolean {
        return this.mService!!.isAppInstalled(appId)
    }

    /// Uninstall an installed app
    fun uninstallApp(appId: String) {
        this.mService!!.uninstallApp(appId)
    }

    /// Enable an installed app
    fun enableApp(appId: String) {
        this.mService!!.enableApp(appId)
    }

    /// Disable an installed app
    fun disableApp(appId: String) {
        this.mService!!.disableApp(appId)
    }

    /// List installed happs in conductor
    fun listInstalledApps(): MutableList<AppInfoFfiAidl> {
        return this.mService!!.listInstalledApps()
    }

    /// Get or create an app websocket with authentication token
    fun appWebsocketAuth(appId: String): AppWebsocketAuthFfiAidl {
        return this.mService!!.appWebsocketAuth(appId)
    }

    fun signZomeCall(args: SignZomeCallRequestAidl): ZomeCallSignedFfiAidl {
        return this.mService!!.signZomeCall(args);
    }
}
