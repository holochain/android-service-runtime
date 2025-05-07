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

    // / Connect to the service
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

    // / Entire process to setup an app
    // /
    // / Check if app is installed.
    // / If not, install it and (optionally) enable it.
    // / Then ensure there is an app websocket available for it
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

    // / Connect to service, wait for connection to be ready, and setupApp
    suspend fun connectSetupApp(
        installAppPayload: InstallAppPayloadFfi,
        enableAfterInstall: Boolean,
    ): AppAuthFfi {
        this.connect(installAppPayload.installedAppId!!)
        this.waitForConnectReady()
        return this.setupApp(installAppPayload, enableAfterInstall)
    }

    // / Enable an installed app
    suspend fun enableApp(installedAppId: String): AppInfoFfi {
        Log.d(logTag, "enableApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        var callbackDeferred = EnableAppCallbackDeferred()
        this.mService!!.enableApp(callbackDeferred)

        return callbackDeferred.await()
    }

    // / Sign a zome call
    suspend fun signZomeCall(args: ZomeCallUnsignedFfi): ZomeCallFfi {
        Log.d(logTag, "signZomeCall")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        var callbackDeferred = SignZomeCallCallbackDeferred()
        this.mService!!.signZomeCall(callbackDeferred, ZomeCallUnsignedFfiParcel(args))

        return callbackDeferred.await()
    }

    // / Poll until we are connected to the service, or the timeout has elapsed
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
