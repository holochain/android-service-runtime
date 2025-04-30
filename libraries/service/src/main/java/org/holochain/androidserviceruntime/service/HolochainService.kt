package org.holochain.androidserviceruntime.service

import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Binder
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import androidx.core.content.ContextCompat
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.holochain.androidserviceruntime.client.AppAuthFfiParcel
import org.holochain.androidserviceruntime.client.AppInfoFfiParcel
import org.holochain.androidserviceruntime.client.IHolochainServiceAdmin
import org.holochain.androidserviceruntime.client.IHolochainServiceApp
import org.holochain.androidserviceruntime.client.IHolochainServiceCallback
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfiParcel
import org.holochain.androidserviceruntime.client.RuntimeConfigFfi
import org.holochain.androidserviceruntime.client.ZomeCallFfiParcel
import org.holochain.androidserviceruntime.client.ZomeCallUnsignedFfiParcel
import java.security.InvalidParameterException

class HolochainService : Service() {
    // / The uniffi-generated holochain runtime bindings
    private val logTag = "HolochainService"
    private var runtime: RuntimeFfi? = null
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val adminBinder by lazy { AdminBinder() }
    private val appBinders = mutableMapOf<String, AppBinder>()
    private var appAuthorizationRequests = AppAuthorizationRequestMap()
    private var uniqueNotificationIdGenerator = UniqueNotificationId()

    companion object {
        // Notification Channel Id for service running notification
        const val NOTIFICATION_CHANNEL_ID_FOREGROUND_SERVICE = "org.holochain.androidserviceruntime.service.FOREGROUND_SERVICE"

        // Notification Channel Id for app authorization notifications
        const val NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION = "org.holochain.androidserviceruntime.service.APP_AUTHORIZATION"

        // Approve app authorization request
        const val ACTION_APPROVE_APP_AUTHORIZATION = "org.holochain.androidserviceruntime.service.APPROVE_APP_AUTHORIZATION"

        // Deny app authorization request
        const val ACTION_DENY_APP_AUTHORIZATION = "org.holochain.androidserviceruntime.service.DENY_APP_AUTHORIZATION"

        // Start the Service, and the Holochain conductor
        const val ACTION_START = "org.holochain.androidserviceruntime.service.ACTION_START"
    }

    private inner class AdminBinder : IHolochainServiceAdmin.Stub() {
        private val logTag = "IHolochainServiceAdmin"

        // / Stop the conductor
        override fun stop() {
            Log.d(logTag, "shutdown")
            this.expectAuthorized()

            this@HolochainService.stopForeground()
        }

        // / Is the conductor started and ready to receive calls
        override fun isReady(): Boolean {
            Log.d(logTag, "isReady")
            this.expectAuthorized()

            return runtime != null
        }

        // / Setup an app
        override fun setupApp(
            callback: IHolochainServiceCallback,
            payload: InstallAppPayloadFfiParcel,
            enableAfterInstall: Boolean,
        ) {
            Log.d(logTag, "setupApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.setupApp(AppAuthFfiParcel(runtime!!.setupApp(payload.inner, enableAfterInstall)))
            }
        }

        // / Install an app
        override fun installApp(
            callback: IHolochainServiceCallback,
            request: InstallAppPayloadFfiParcel,
        ) {
            Log.d(logTag, "installApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.installApp(AppInfoFfiParcel(runtime!!.installApp(request.inner)))
            }
        }

        // / Uninstall an app
        override fun uninstallApp(
            callback: IHolochainServiceCallback,
            installedAppId: String,
        ) {
            Log.d(logTag, "uninstallApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                runtime!!.uninstallApp(installedAppId)
                callback.uninstallApp()
            }
        }

        // / Enable an installed app
        override fun enableApp(
            callback: IHolochainServiceCallback,
            installedAppId: String,
        ) {
            Log.d(logTag, "enableApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.enableApp(AppInfoFfiParcel(runtime!!.enableApp(installedAppId)))
            }
        }

        // / Disable an installed app
        override fun disableApp(
            callback: IHolochainServiceCallback,
            installedAppId: String,
        ) {
            Log.d(logTag, "disableApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                runtime!!.disableApp(installedAppId)
                callback.disableApp()
            }
        }

        // / List installed apps
        override fun listApps(callback: IHolochainServiceCallback) {
            Log.d(logTag, "listApps")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.listApps(runtime!!.listApps().map { AppInfoFfiParcel(it) })
            }
        }

        // / Is app installed
        override fun isAppInstalled(
            callback: IHolochainServiceCallback,
            installedAppId: String,
        ) {
            Log.d(logTag, "isAppInstalled")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.isAppInstalled(runtime!!.isAppInstalled(installedAppId))
            }
        }

        // / Get or create an app websocket with an authenticated token
        override fun ensureAppWebsocket(
            callback: IHolochainServiceCallback,
            installedAppId: String,
        ) {
            Log.d(logTag, "ensureAppWebsocket")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.ensureAppWebsocket(AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!))
            }
        }

        // / Sign a zome call
        override fun signZomeCall(
            callback: IHolochainServiceCallback,
            req: ZomeCallUnsignedFfiParcel,
        ) {
            Log.d(logTag, "signZomeCall")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }

        // We cannot call Binder.getCallingUid() within the onBind callback,
        // so instead we check the authorization within each IPC call
        private fun expectAuthorized() {
            val clientUid = Binder.getCallingUid()
            val clientPackageName =
                HolochainService@ packageManager.getNameForUid(clientUid)
                    ?: throw UnauthorizedException(
                        "Unable to get name of package binding to AdminBinder uid=$clientUid",
                    )

            Log.d(logTag, "AdminBinder expectAuthorized clientPackageName=$clientPackageName")

            if (clientPackageName != HolochainService@ packageName) {
                throw UnauthorizedException(
                    "Package is not authorized to access the AdminBinder clientPackageName=$clientPackageName",
                )
            }
        }
    }

    private inner class AppBinder(
        private val installedAppId: String,
    ) : IHolochainServiceApp.Stub() {
        private val logTag = "IHolochainServiceApp installedAppId=$installedAppId"

        // / Setup an app
        override fun setupApp(
            callback: IHolochainServiceCallback,
            payload: InstallAppPayloadFfiParcel,
            enableAfterInstall: Boolean,
        ) {
            Log.d(logTag, "setupApp")
            this.expectAuthorized()

            if (payload.inner.installedAppId != installedAppId) {
                throw UnauthorizedException(
                    "Only the installedAppId specified in the AppBinder can be installed: expected $installedAppId but received ${payload.inner.installedAppId}",
                )
            }

            serviceScope.launch(Dispatchers.IO) {
                callback.setupApp(AppAuthFfiParcel(runtime!!.setupApp(payload.inner, enableAfterInstall)))
            }
        }

        // / Enable an installed app
        override fun enableApp(callback: IHolochainServiceCallback) {
            Log.d(logTag, "enableApp")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.enableApp(AppInfoFfiParcel(runtime!!.enableApp(installedAppId)))
            }
        }

        // / Get or create an app websocket with an authenticated token
        override fun ensureAppWebsocket(callback: IHolochainServiceCallback) {
            Log.d(logTag, "ensureAppWebsocket")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.ensureAppWebsocket(AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!))
            }
        }

        // / Sign a zome call
        override fun signZomeCall(
            callback: IHolochainServiceCallback,
            req: ZomeCallUnsignedFfiParcel,
        ) {
            Log.d(logTag, "signZomeCall")
            this.expectAuthorized()

            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }

        // We cannot call Binder.getCallingUid() within the onBind callback,
        // so instead we check the authorization within each IPC call
        private fun expectAuthorized() {
            val clientUid = Binder.getCallingUid()
            val clientPackageName =
                packageManager.getNameForUid(clientUid)
                    ?: throw UnauthorizedException(
                        "Unable to get name of package binding to AdminBinder uid=$clientUid",
                    )

            Log.d(
                logTag,
                "AppBinder expectAuthorized clientPackageName=$clientPackageName installedAppId=$installedAppId",
            )
            var isAuthorized = runtime!!.isAppClientAuthorized(clientPackageName, installedAppId)

            if (!isAuthorized) {
                // Show notification asking user to authorize this app client
                HolochainService@ showAppAuthorizationNotification(clientPackageName, installedAppId)

                throw UnauthorizedException(
                    "Package is not authorized to access the AppBinder clientPackageName=$clientPackageName installedAppId=${this.installedAppId}",
                )
            }
        }
    }

    internal fun authorizeAppClient(
        clientPackageName: String,
        installedAppId: String,
    ) {
        runtime!!.authorizeAppClient(clientPackageName, installedAppId)
    }

    private fun showAppAuthorizationNotification(
        clientPackageName: String,
        installedAppId: String,
    ) {
        Log.d(
            logTag,
            "showAppAuthorizationNotification clientPackageName=$clientPackageName installedAppId=$installedAppId",
        )

        // Create approve intent
        val requestId = appAuthorizationRequests.put(AppAuthorizationRequest(clientPackageName, installedAppId))
        val approveIntent =
            Intent(this, HolochainService::class.java).apply {
                action = ACTION_APPROVE_APP_AUTHORIZATION
                putExtra("requestId", requestId)
            }
        val approvePendingIntent =
            PendingIntent.getService(
                this,
                2,
                approveIntent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
            )

        // Create deny intent
        val denyIntent =
            Intent(this, HolochainService::class.java).apply {
                action = ACTION_DENY_APP_AUTHORIZATION
                putExtra("requestId", requestId)
            }
        val denyPendingIntent =
            PendingIntent.getService(
                this,
                3,
                denyIntent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
            )

        // Build the notification
        val notification =
            NotificationCompat
                .Builder(this, "HolochainServiceChannel")
                .setContentTitle("App Authorization Request")
                .setStyle(
                    NotificationCompat
                        .BigTextStyle()
                        .bigText("$clientPackageName is requesting access to Holochain app: $installedAppId"),
                ).setSmallIcon(R.drawable.notification_icon_sm)
                .setPriority(NotificationCompat.PRIORITY_HIGH)
                .setAutoCancel(true)
                .addAction(android.R.drawable.ic_menu_send, "Approve", approvePendingIntent)
                .addAction(android.R.drawable.ic_menu_close_clear_cancel, "Deny", denyPendingIntent)
                .build()

        // Show the notification
        NotificationManagerCompat.from(this).notify(uniqueNotificationIdGenerator.get(), notification)
    }

    override fun onStartCommand(
        intent: Intent?,
        flags: Int,
        startId: Int,
    ): Int {
        Log.d(logTag, "onStartCommand")

        if (intent?.action != null) {
            when (intent.action) {
                ACTION_START -> {
                    Log.d(
                        logTag,
                        "onStartCommand ACTION_START",
                    )
                    startForeground()
                }
                ACTION_APPROVE_APP_AUTHORIZATION -> {
                    Log.d(logTag, "onStartCommand ACTION_APPROVE_APP_AUTHORIZATION")

                    val requestId = intent.getStringExtra("requestId")
                    if (requestId != null) {
                        val request = appAuthorizationRequests.pop(requestId)
                        if (request != null) {
                            authorizeAppClient(request.clientPackageName, request.installedAppId)
                        }
                    }
                }
                ACTION_DENY_APP_AUTHORIZATION -> {
                    Log.d(logTag, "onStartCommand ACTION_DENY_APP_AUTHORIZATION, ignoring")

                    val requestId = intent.getStringExtra("requestId")
                    if (requestId != null) {
                        val request = appAuthorizationRequests.pop(requestId)
                    }
                }
            }
        }

        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
    }

    override fun onBind(intent: Intent): IBinder? {
        var api = intent.getStringExtra("api")
        Log.d(logTag, "onBind: api=$api action=${intent.action}")

        when (api) {
            "admin" -> {
                return adminBinder
            }
            "app" -> {
                var installedAppId =
                    intent.getStringExtra("installedAppId")
                        ?: throw InvalidParameterException(
                            "When binding with api=app, installedAppId must be provided",
                        )

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
            val notification =
                NotificationCompat
                    .Builder(this, NOTIFICATION_CHANNEL_ID_FOREGROUND_SERVICE)
                    .setContentTitle("Holochain Service")
                    .setContentText("Holochain Service is running")
                    .setSmallIcon(R.drawable.notification_icon_sm)
                    .build()
            startForeground(1, notification)

            // Start holochain conductor
            val passphrase = byteArrayOf(0x48, 101, 108, 108, 111)
            val config =
                RuntimeConfigFfi(
                    getFilesDir().toString(),
                    "https://bootstrap-0.infra.holochain.org",
                    "wss://sbd.holo.host",
                )

            serviceScope.launch(Dispatchers.IO) {
                runtime = RuntimeFfi.start(passphrase, config)
                Log.d(logTag, "Holochain started successfully")
            }
        } catch (e: Exception) {
            Log.e(logTag, "Holochain failed to start $e")
        }
    }

    fun stopForeground() {
        // Shutdown conductor
        runBlocking { runtime?.stop() }
        runtime = null

        // Stop service
        super.stopForeground(true)
        stopSelf()
    }
}
