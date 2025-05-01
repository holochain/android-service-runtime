package org.holochain.androidserviceruntime.client

import android.app.Activity
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.ServiceConnection
import android.os.IBinder
import android.util.Log
import kotlinx.coroutines.delay

class HolochainServiceAdminClient(
    private val activity: Activity,
    private val serviceComponentName: ComponentName,
) {
    private var mService: IHolochainServiceAdmin? = null
    private val logTag = "HolochainServiceAdminClient"

    // IPC Connection to HolochainService using AIDL
    private val mConnection =
        object : ServiceConnection {
            override fun onServiceConnected(
                className: ComponentName,
                service: IBinder,
            ) {
                mService = IHolochainServiceAdmin.Stub.asInterface(service)
                Log.d(logTag, "IHolochainServiceAdmin connected")
            }

            override fun onServiceDisconnected(className: ComponentName) {
                mService = null
                Log.d(logTag, "IHolochainServiceAdmin disconnected")
            }
        }

    // / Connect to the service
    fun connect() {
        Log.d(logTag, "connect")

        // Giving the intent a unique action ensures the HolochainService `onBind()` callback is
        // triggered.
        val packageName: String = this.activity.getPackageName()
        val intent = Intent(packageName)

        intent.putExtra("api", "admin")
        intent.setComponent(this.serviceComponentName)
        this.activity.bindService(intent, this.mConnection, Context.BIND_ABOVE_CLIENT)
    }

    // / Full process to setup an app
    // /
    // / Check if app is installed, if not install it, then optionally enable it.
    // / Then ensure there is an app websocket and authentication for it.
    // /
    // / If an app is already installed, it will not be enabled. It is only enabled after a successful
    // install.
    // / The reasoning is that if an app is disabled after that point,
    // / it is assumed to have been manually disabled in the admin interface, which we don't want to
    // override.
    suspend fun setupApp(
        installAppPayload: InstallAppPayloadFfi,
        enableAfterInstall: Boolean,
    ): AppAuthFfi {
        Log.d(logTag, "setupApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = SetupAppCallbackDeferred()
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
        this.connect()
        this.waitForConnectReady()
        return this.setupApp(installAppPayload, enableAfterInstall)
    }

    // / Stop the service
    fun stop() {
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }
        this.mService!!.stop()
    }

    // / Is the service ready to receive calls
    fun isReady(): Boolean {
        if (this.mService == null) {
            return false
        }
        return this.mService!!.isReady()
    }

    // / Install an app
    suspend fun installApp(payload: InstallAppPayloadFfi): AppInfoFfi {
        Log.d(logTag, "installApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = InstallAppCallbackDeferred()
        this.mService!!.installApp(callbackDeferred, InstallAppPayloadFfiParcel(payload))

        return callbackDeferred.await()
    }

    // / Is an app with the given app_id installed
    suspend fun isAppInstalled(installedAppId: String): Boolean {
        Log.d(logTag, "isAppInstalled")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = IsAppInstalledCallbackDeferred()
        this.mService!!.isAppInstalled(callbackDeferred, installedAppId)

        return callbackDeferred.await()
    }

    // / Uninstall an installed app
    suspend fun uninstallApp(installedAppId: String) {
        Log.d(logTag, "uninstallApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = UninstallAppCallbackDeferred()
        this.mService!!.uninstallApp(callbackDeferred, installedAppId)

        callbackDeferred.await()
    }

    // / Enable an installed app
    suspend fun enableApp(installedAppId: String): AppInfoFfi {
        Log.d(logTag, "enableApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = EnableAppCallbackDeferred()
        this.mService!!.enableApp(callbackDeferred, installedAppId)
        return callbackDeferred.await()
    }

    // / Disable an installed app
    suspend fun disableApp(installedAppId: String) {
        Log.d(logTag, "disableApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = DisableAppCallbackDeferred()
        this.mService!!.disableApp(callbackDeferred, installedAppId)
        callbackDeferred.await()
    }

    // / List installed happs in conductor
    suspend fun listApps(): List<AppInfoFfi> {
        Log.d(logTag, "listApps")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = ListAppsCallbackDeferred()
        this.mService!!.listApps(callbackDeferred)

        return callbackDeferred.await()
    }

    // / Get or create an app websocket with authentication token
    suspend fun ensureAppWebsocket(installedAppId: String): AppAuthFfi {
        Log.d(logTag, "ensureAppWebsocket")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = EnsureAppWebsocketCallbackDeferred()
        this.mService!!.ensureAppWebsocket(callbackDeferred, installedAppId)

        return callbackDeferred.await()
    }

    // / Sign a zome call
    suspend fun signZomeCall(args: ZomeCallUnsignedFfi): ZomeCallFfi {
        Log.d(logTag, "signZomeCall")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = SignZomeCallCallbackDeferred()
        this.mService!!.signZomeCall(callbackDeferred, ZomeCallUnsignedFfiParcel(args))

        return callbackDeferred.await()
    }

    // / Poll until we are connected to the service, or the timeout has elapsed
    suspend fun waitForConnectReady(timeoutMs: Long = 100L) {
        var intervalMs = 5L
        var elapsedMs = 0L
        while (elapsedMs <= timeoutMs) {
            Log.d(logTag, "waitForConnectReady " + elapsedMs)
            if (this.mService != null) break

            delay(intervalMs)
            elapsedMs += intervalMs
        }
    }
}
