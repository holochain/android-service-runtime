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

    /**
     * Start the Holochain service
     *
     * This must be called before `connect()` can be called.
     */
    fun start(config: RuntimeNetworkConfigFfi) {
        val intent = Intent(HolochainService.ACTION_START)
        intent.setComponent(this.serviceComponentName)
        intent.setExtra("config", config)
        
        this.activity.startForegroundService(intent)
    }
    
    /**
     * Connect to the Holochain service using the Admin API.
     *
     * This must be called before using any functions on the Admin API binder.
     */
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

    /**
     * Full process to setup a Holochain app.
     *
     * This function will:
     * 1. Check if the app is installed
     * 2. If not installed, install it
     * 3. Optionally optionally enable it
     * 4. Ensure there is an app websocket and authentication for it
     *
     * If an app is already installed, it will not be enabled automatically. It is only enabled after
     * a successful install. The reasoning is that if an app is disabled after initial installation,
     * it is assumed to have been manually disabled via the admin UI, which we don't want to override.
     *
     * @param installAppPayload The payload containing app installation data
     * @param enableAfterInstall Whether to enable the app after installation
     * @return AppAuthFfi object containing app websocket authentication and connection information
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

        val callbackDeferred = SetupAppCallbackDeferred()
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
        this.connect()
        this.waitForConnectReady()
        return this.setupApp(installAppPayload, enableAfterInstall)
    }

    /**
     * Stops the Holochain service.
     *
     * This will shutdown the Holochain conductor and stop the service process.
     *
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    fun stop() {
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }
        this.mService!!.stop()
    }

    /**
     * Checks if the service is ready to receive calls.
     *
     * @return true if connected to the service and the Holochain conductor is running, false otherwise
     */
    fun isReady(): Boolean {
        if (this.mService == null) {
            return false
        }
        return this.mService!!.isReady()
    }

    /**
     * Installs a Holochain app
     *
     * @param payload The installation payload containing the app bundle and configuration
     * @return AppInfoFfi object with information about the installed app
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun installApp(payload: InstallAppPayloadFfi): AppInfoFfi {
        Log.d(logTag, "installApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = InstallAppCallbackDeferred()
        this.mService!!.installApp(callbackDeferred, InstallAppPayloadFfiParcel(payload))

        return callbackDeferred.await()
    }

    /**
     * Checks if an app with the given app ID is installed.
     *
     * @param installedAppId The ID of the app to check
     * @return true if the app is installed, false otherwise
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun isAppInstalled(installedAppId: String): Boolean {
        Log.d(logTag, "isAppInstalled")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = IsAppInstalledCallbackDeferred()
        this.mService!!.isAppInstalled(callbackDeferred, installedAppId)

        return callbackDeferred.await()
    }

    /**
     * Uninstalls an installed Holochain app.
     *
     * @param installedAppId The ID of the app to uninstall
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun uninstallApp(installedAppId: String) {
        Log.d(logTag, "uninstallApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = UninstallAppCallbackDeferred()
        this.mService!!.uninstallApp(callbackDeferred, installedAppId)

        callbackDeferred.await()
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

        val callbackDeferred = EnableAppCallbackDeferred()
        this.mService!!.enableApp(callbackDeferred, installedAppId)
        return callbackDeferred.await()
    }

    /**
     * Disables an installed Holochain app.
     *
     * @param installedAppId The ID of the app to disable
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun disableApp(installedAppId: String) {
        Log.d(logTag, "disableApp")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = DisableAppCallbackDeferred()
        this.mService!!.disableApp(callbackDeferred, installedAppId)
        callbackDeferred.await()
    }

    /**
     * Lists all installed Holochain apps in the conductor.
     *
     * @return List of AppInfoFfi objects containing information about all installed apps
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun listApps(): List<AppInfoFfi> {
        Log.d(logTag, "listApps")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = ListAppsCallbackDeferred()
        this.mService!!.listApps(callbackDeferred)

        return callbackDeferred.await()
    }

    /**
     * Gets or creates an app websocket with authentication token.
     *
     * @param installedAppId The ID of the app to create a websocket for
     * @return AppAuthFfi object containing websocket URL and authentication information
     * @throws HolochainServiceNotConnectedException if not connected to the service
     */
    suspend fun ensureAppWebsocket(installedAppId: String): AppAuthFfi {
        Log.d(logTag, "ensureAppWebsocket")
        if (this.mService == null) {
            throw HolochainServiceNotConnectedException()
        }

        val callbackDeferred = EnsureAppWebsocketCallbackDeferred()
        this.mService!!.ensureAppWebsocket(callbackDeferred, installedAppId)

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

        val callbackDeferred = SignZomeCallCallbackDeferred()
        this.mService!!.signZomeCall(callbackDeferred, ZomeCallUnsignedFfiParcel(args))

        return callbackDeferred.await()
    }

    /**
     * Polls until connected to the service, or the timeout has elapsed.
     *
     * @param timeoutMs Maximum time to wait for connection in milliseconds (default: 100ms)
     */
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
