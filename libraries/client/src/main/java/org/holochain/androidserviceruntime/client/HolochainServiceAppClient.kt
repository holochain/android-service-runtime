package org.holochain.androidserviceruntime.client

import android.app.Activity
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.util.Log
import kotlinx.coroutines.delay

class HolochainServiceAppClient(
    private val activity: Activity,
    private val serviceComponentName: ComponentName,
) {
    private var mService: IHolochainServiceApp? = null
    private val logTag = "HolochainServiceAppClient"

    // IPC Connection to HolochainService using AIDL
    private val mConnection =
        object : ServiceConnection {
            override fun onServiceConnected(
                className: ComponentName,
                service: IBinder,
            ) {
                mService = IHolochainServiceApp.Stub.asInterface(service)
                Log.d(logTag, "IHolochainServiceApp connected")
            }

            override fun onServiceDisconnected(className: ComponentName) {
                mService = null
                Log.d(logTag, "IHolochainServiceApp disconnected")
            }
        }

    /**
     * Connects to the Holochain service using the App API for a specific app.
     * 
     * @param installedAppId The ID of the app to connect to
     */
    fun connect(installedAppId: String) {
        Log.d(logTag, "connect")

        // Giving the intent a unique action ensures the HolochainService `onBind()` callback is
        // triggered.
        val packageName: String = this.activity.getPackageName()
        val intent = Intent("$packageName:$installedAppId")

        intent.putExtra("api", "app")
        intent.putExtra("installedAppId", installedAppId)
        intent.setComponent(this.serviceComponentName)
        this.activity.bindService(intent, this.mConnection, Context.BIND_ABOVE_CLIENT)
    }

    /**
     * Complete process to setup a Holochain app.
     * 
     * This method will:
     * 1. Check if the app is installed
     * 2. If not installed, install it
     * 3. Optionally enable it
     * 4. Ensure there is an app websocket available for it
     *
     * If an app is already installed, it will not be enabled automatically. It is only enabled after
     * a successful install. The reasoning is that if an app is disabled after initial installation,
     *
     * @param installAppPayload The payload containing app installation data
     * @param enableAfterInstall Whether to enable the app after installation
     * @return AppAuthFfi object containing authentication and connection information
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun setupApp(
        installAppPayload: InstallAppPayloadFfi,
        enableAfterInstall: Boolean,
    ): AppAuthFfi {
        Log.d(logTag, "setupApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        var callbackDeferred = SetupAppCallbackDeferred()
        this.mService!!.setupApp(
            callbackDeferred,
            InstallAppPayloadFfiParcel(installAppPayload),
            enableAfterInstall,
        )

        return callbackDeferred.await()
    }

    /**
     * Connects to service, waits for connection to be ready, and sets up an app.
     * 
     * Convenience method that combines connect(), waitForConnectReady(), and setupApp() into a single call.
     *
     * @param installAppPayload The payload containing app installation data
     * @param enableAfterInstall Whether to enable the app after installation
     * @return AppAuthFfi object containing authentication and connection information
     */
    suspend fun connectSetupApp(
        installAppPayload: InstallAppPayloadFfi,
        enableAfterInstall: Boolean,
    ): AppAuthFfi {
        this.connect(installAppPayload.installedAppId!!)
        this.waitForConnectReady()
        return this.setupApp(installAppPayload, enableAfterInstall)
    }

    /**
     * Enables an installed Holochain app.
     * 
     * @param installedAppId The ID of the app to enable
     * @return AppInfoFfi object with information about the enabled app
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun enableApp(installedAppId: String): AppInfoFfi {
        Log.d(logTag, "enableApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        var callbackDeferred = EnableAppCallbackDeferred()
        this.mService!!.enableApp(callbackDeferred)

        return callbackDeferred.await()
    }

    /**
     * Signs a zome call with the agent's private key.
     * 
     * This is required for making authenticated calls to Holochain zome functions.
     * 
     * @param args The unsigned zome call to sign
     * @return The signed zome call ready to be executed
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun signZomeCall(args: ZomeCallUnsignedFfi): ZomeCallFfi {
        Log.d(logTag, "signZomeCall")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        var callbackDeferred = SignZomeCallCallbackDeferred()
        this.mService!!.signZomeCall(callbackDeferred, ZomeCallUnsignedFfiParcel(args))

        return callbackDeferred.await()
    }

    /**
     * Polls until connected to the service, or the timeout has elapsed.
     * 
     * @param timeoutMs Maximum time to wait for connection in milliseconds (default: 100ms)
     * @param intervalMs Time between connection checks in milliseconds (default: 5ms)
     */
    private suspend fun waitForConnectReady(
        timeoutMs: Long = 100L,
        intervalMs: Long = 5L,
    ) {
        var elapsedMs = 0L
        while (elapsedMs <= timeoutMs) {
            Log.d(logTag, "waitForConnectReady " + elapsedMs)
            if (this.mService != null) break

            delay(intervalMs)
            elapsedMs += intervalMs
        }
    }
}
