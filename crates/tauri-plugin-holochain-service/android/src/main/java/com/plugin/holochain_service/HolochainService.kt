package com.plugin.holochain_service

import android.util.Log
import android.app.Service
import android.os.IBinder
import android.content.Intent
import android.os.Binder
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import com.plugin.holochain_service.holochain_conductor_runtime_ffi.RuntimeFfi
import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceAdmin
import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceApp
import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceCallback
import org.holochain.androidserviceruntime.holochain_service_client.RuntimeConfigFfi
import org.holochain.androidserviceruntime.holochain_service_client.AppInfoFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.AppAuthFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.InstallAppPayloadFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallFfiParcel
import java.security.InvalidParameterException

class HolochainService : Service() {
    /// The uniffi-generated holochain runtime bindings
    private val TAG = "HolochainService"
    private var runtime: RuntimeFfi? = null
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val adminBinder by lazy { AdminBinder() }
    private val appBinders = mutableMapOf<String, AppBinder>()

    private inner class AdminBinder : IHolochainServiceAdmin.Stub() {
        private val TAG = "IHolochainServiceAdmin"

        /// Stop the conductor
        override fun stop() {
            Log.d(TAG, "shutdown")
            this.expectAuthorized()

            this@HolochainService.stopForeground()
        }

        /// Is the conductor started and ready to receive calls
        override fun isReady(): Boolean {
            Log.d(TAG, "isReady")
            this.expectAuthorized()

            return runtime != null
        }

        /// Setup an app
        override fun setupApp(
            callback: IHolochainServiceCallback,
            payload: InstallAppPayloadFfiParcel,
            enableAfterInstall: Boolean
        ) {
            Log.d(TAG, "setupApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.setupApp(AppAuthFfiParcel(runtime!!.setupApp(payload.inner, enableAfterInstall)))
            }
        }
        
        /// Install an app
        override fun installApp(
            callback: IHolochainServiceCallback,
            request: InstallAppPayloadFfiParcel
        ) {
            Log.d(TAG, "installApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.installApp(AppInfoFfiParcel(runtime!!.installApp(request.inner)))
            }
        }

        /// Uninstall an app
        override fun uninstallApp(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "uninstallApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                runtime!!.uninstallApp(installedAppId)
                callback.uninstallApp()
            }
        }

        /// Enable an installed app
        override fun enableApp(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "enableApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.enableApp(AppInfoFfiParcel(runtime!!.enableApp(installedAppId)))
            }
        }

        /// Disable an installed app
        override fun disableApp(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "disableApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                runtime!!.disableApp(installedAppId)
                callback.disableApp()
            }
        }

        /// List installed apps
        override fun listApps(callback: IHolochainServiceCallback) {
            Log.d(TAG, "listApps")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.listApps(runtime!!.listApps().map {
                    AppInfoFfiParcel(it)
                })
            }
        }

        /// Is app installed
        override fun isAppInstalled(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "isAppInstalled")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.isAppInstalled(runtime!!.isAppInstalled(installedAppId))
            }
        }

        /// Get or create an app websocket with an authenticated token
        override fun ensureAppWebsocket(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "ensureAppWebsocket")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.ensureAppWebsocket(AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!))
            }
        }

        /// Sign a zome call
        override fun signZomeCall(
            callback: IHolochainServiceCallback,
            req: ZomeCallUnsignedFfiParcel
        ) {
            Log.d(TAG, "signZomeCall")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }

        // We cannot call Binder.getCallingUid() within the onBind callback,
        // so instead we check the authorization within each IPC call
        private fun expectAuthorized() {
            val clientUid = Binder.getCallingUid()
            val clientPackageName = HolochainService@packageManager.getNameForUid(clientUid)
                ?: throw UnauthorizedException("Unable to get name of package binding to AdminBinder uid=$clientUid")

            Log.d(TAG, "AdminBinder expectAuthorized clientPackageName=$clientPackageName")

            if (clientPackageName != HolochainService@packageName){
                // TODO notify user to request authorization

                throw UnauthorizedException("Package is not authorized to access the AdminBinder clientPackageName=$clientPackageName")
            }
        }
    }

    private inner class AppBinder(private val installedAppId: String) : IHolochainServiceApp.Stub() {
        private val TAG = "IHolochainServiceApp installedAppId=$installedAppId"

        /// Setup an app
        override fun setupApp(
            callback: IHolochainServiceCallback,
            payload: InstallAppPayloadFfiParcel,
            enableAfterInstall: Boolean
        ) {
            Log.d(TAG, "setupApp")
            this.expectAuthorized(installedAppId)

            serviceScope.launch(Dispatchers.IO) {
                callback.setupApp(AppAuthFfiParcel(runtime!!.setupApp(payload.inner, enableAfterInstall)))
            }
        }
    
        /// Enable an installed app
        override fun enableApp(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "enableApp")
            this.expectAuthorized(installedAppId)

            serviceScope.launch(Dispatchers.IO) {
                callback.enableApp(AppInfoFfiParcel(runtime!!.enableApp(installedAppId)))
            }
        }

        /// Get or create an app websocket with an authenticated token
        override fun ensureAppWebsocket(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "ensureAppWebsocket")
            this.expectAuthorized(installedAppId)

            serviceScope.launch(Dispatchers.IO) {
                callback.ensureAppWebsocket(AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!))
            }
        }

        /// Sign a zome call
        override fun signZomeCall(
            callback: IHolochainServiceCallback,
            req: ZomeCallUnsignedFfiParcel
        ) {
            Log.d(TAG, "signZomeCall")
            this.expectAuthorized(installedAppId)

            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }

        // We cannot call Binder.getCallingUid() within the onBind callback,
        // so instead we check the authorization within each IPC call
        private fun expectAuthorized(installedAppId: String) {
            val clientUid = Binder.getCallingUid()
            val clientPackageName = packageManager.getNameForUid(clientUid)
                ?: throw UnauthorizedException("Unable to get name of package binding to AdminBinder uid=$clientUid")

            Log.d(TAG, "AppBinder expectAuthorized clientPackageName=$clientPackageName installedAppId=$installedAppId")
            var isAuthorized = runtime!!.isAppClientAuthorized(
                clientPackageName,
                installedAppId
            )

            if (!isAuthorized){
                Log.d(TAG, "authorizeAppClient clientPackageName=$clientPackageName installedAppId=$installedAppId")
                // TODO notify user to request authorization
                runtime!!.authorizeAppClient(
                    clientPackageName,
                    installedAppId
                )

                // TODO If user does not provide authorization
                // throw UnauthorizedException("Package is not authorized to access the AppBinder clientPackageName=$clientPackageName installedAppId=$installedAppId")
            }
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground()
        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
    }

    override fun onBind(intent: Intent): IBinder? {
        var api = intent.getStringExtra("api")
        Log.d(TAG, "onBind: api=$api action=${intent.action}")

        when (api) {
            "admin" -> {
                return adminBinder
            }
            "app" -> {
                var installedAppId = intent.getStringExtra("installedAppId")
                    ?: throw InvalidParameterException("When binding with api=app, installedAppId must be provided")

                return this.getOrCreateAppBinder(installedAppId)
            }
            else -> {
                throw InvalidParameterException("Intent extra 'api' must be 'admin' or 'app'")
            }
        }
    }

    // Get or create an AppServiceBinder for this app
    private fun getOrCreateAppBinder(installedAppId: String): AppBinder {
        if (!this.appBinders.containsKey(installedAppId)) {
            this.appBinders[installedAppId] = AppBinder(installedAppId)
        }
        return this.appBinders[installedAppId]!!
    }

    private fun startForeground() {
        try {
            // Create the notification to display while the service is running
            val notification = NotificationCompat.Builder(this, "HolochainServiceChannel")
                .setContentTitle("Holochain Service Runtime")
                .setContentText("Holochain Service is running")
                .setSmallIcon(R.drawable.notification_icon_sm)
                .build()
            startForeground(1, notification)

            // Start holochain conductor
            val passphrase = byteArrayOf(0x48, 101, 108, 108, 111)
            val config = RuntimeConfigFfi(
                getFilesDir().toString(),
                "https://bootstrap-0.infra.holochain.org",
                "wss://sbd.holo.host",
            );
            
            serviceScope.launch(Dispatchers.IO) {
                runtime = RuntimeFfi.start(passphrase, config)
                Log.d(TAG, "Holochain started successfully")
            }
        } catch (e: Exception) {
           Log.e(TAG, "Holochain failed to start $e")
        }
    }

    fun stopForeground() {
        // Shutdown conductor
        runBlocking {
            runtime?.stop()
        }
        runtime = null

        // Stop service
        super.stopForeground(true)
        stopSelf()
    }
}
