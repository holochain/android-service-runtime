package org.holochain.androidserviceruntime.service

import android.app.PendingIntent
import android.app.Service
import android.content.Intent
import android.content.pm.PackageManager
import android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_DATA_SYNC
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
import org.holochain.androidserviceruntime.client.AdminBinderUnauthorizedException
import org.holochain.androidserviceruntime.client.AdminBinderUnauthorizedExceptionParcel
import org.holochain.androidserviceruntime.client.AppAuthFfiParcel
import org.holochain.androidserviceruntime.client.AppBinderUnauthorizedException
import org.holochain.androidserviceruntime.client.AppBinderUnauthorizedExceptionParcel
import org.holochain.androidserviceruntime.client.AppInfoFfiParcel
import org.holochain.androidserviceruntime.client.HolochainServiceIntentActions
import org.holochain.androidserviceruntime.client.IHolochainServiceAdmin
import org.holochain.androidserviceruntime.client.IHolochainServiceApp
import org.holochain.androidserviceruntime.client.IHolochainServiceCallback
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfiParcel
import org.holochain.androidserviceruntime.client.RuntimeConfigFfi
import org.holochain.androidserviceruntime.client.RuntimeNetworkConfigFfi
import org.holochain.androidserviceruntime.client.RuntimeNetworkConfigFfiParcel
import org.holochain.androidserviceruntime.client.ZomeCallParamsFfiParcel
import org.holochain.androidserviceruntime.client.ZomeCallParamsSignedFfiParcel
import java.security.InvalidParameterException

class HolochainService : Service() {
    // / The uniffi-generated holochain runtime bindings
    private val logTag = "HolochainService"
    private var autostartConfigManager: AutostartConfigManagerFfi? = null
    private var runtime: RuntimeFfi? = null
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val adminBinder by lazy { AdminBinder() }
    private val appBinders = mutableMapOf<String, AppBinder>()
    private var appAuthorizationRequests = AppAuthorizationRequestMap(NOTIFICATION_ID_FOREGROUND_SERVICE_RUNNING + 1)

    companion object {
        // Reserved notification id for service running notification
        const val NOTIFICATION_ID_FOREGROUND_SERVICE_RUNNING = 1

        // Notification Channel Id for service running notification
        const val NOTIFICATION_CHANNEL_ID_FOREGROUND_SERVICE = "org.holochain.androidserviceruntime.service.FOREGROUND_SERVICE"

        // Notification Channel Id for app authorization notifications
        const val NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION = "org.holochain.androidserviceruntime.service.APP_AUTHORIZATION"
    }

    /**
     * Called by the system when the service is first created or when an intent is delivered to the service.
     *
     * Handles several actions:
     * - ACTION_START: Enables autostart and starts the Holochain service
     * - ACTION_BOOT_COMPLETED: Starts the service if autostart is enabled
     * - ACTION_APPROVE_APP_AUTHORIZATION: Authorizes an app to access Holochain
     * - ACTION_DENY_APP_AUTHORIZATION: Dismisses an authorization request
     *
     * @param intent The Intent supplied to startService(Intent)
     * @param flags Additional data about this start request
     * @param startId A unique integer representing this specific request to start
     * @return How to continue running the service (NOT_STICKY in this case)
     */
    override fun onStartCommand(
        intent: Intent?,
        flags: Int,
        startId: Int,
    ): Int {
        Log.d(logTag, "onStartCommand")

        when (intent?.action) {
            HolochainServiceIntentActions.ACTION_START -> {
                Log.d(
                    logTag,
                    "onStartCommand ACTION_START",
                )

                // Get config from intent
                val config = intent.extras?.getParcelable<RuntimeNetworkConfigFfiParcel>("config")
                if (config != null) {
                    // Enable conductor to autostart on system boot
                    getAutostartConfigManager().enable(config.inner)

                    // Start service
                    startForeground(config.inner)
                }
            }
            HolochainServiceIntentActions.ACTION_BOOT_COMPLETED -> {
                Log.d(
                    logTag,
                    "onStartCommand ACTION_BOOT_COMPLETED",
                )

                // If autostart is enabled, start service
                val config = getAutostartConfigManager().getEnabledConfig()
                if (config != null) {
                    startForeground(config)
                }
            }
            HolochainServiceIntentActions.ACTION_APPROVE_APP_AUTHORIZATION -> {
                Log.d(logTag, "onStartCommand ACTION_APPROVE_APP_AUTHORIZATION")

                val requestId = intent.getStringExtra("requestId")
                if (requestId != null) {
                    val request = appAuthorizationRequests.pop(requestId)
                    if (request != null) {
                        NotificationManagerCompat.from(this).cancel(NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION, request.notificationId)
                        runtime!!.authorizeAppClient(request.request.clientPackageName, request.request.installedAppId)
                    }
                }
            }
            HolochainServiceIntentActions.ACTION_DENY_APP_AUTHORIZATION -> {
                Log.d(logTag, "onStartCommand ACTION_DENY_APP_AUTHORIZATION, ignoring")

                val requestId = intent.getStringExtra("requestId")
                if (requestId != null) {
                    val request = appAuthorizationRequests.pop(requestId)
                    if (request != null) {
                        NotificationManagerCompat.from(this).cancel(NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION, request.notificationId)
                    }
                }
            }
        }

        return START_NOT_STICKY
    }

    override fun onDestroy() {
        super.onDestroy()
    }

    /**
     * Called when a client binds to the service with bindService().
     *
     * Returns either an AdminBinder or AppBinder based on the "api" extra in the intent.
     * For AppBinder, also requires an "installedAppId" extra.
     *
     * @param intent The Intent passed to bindService()
     * @return An IBinder through which clients can call on to the service
     * @throws InvalidParameterException if required parameters are missing or invalid
     */
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

    /**
     * Stops the foreground service and shuts down the Holochain conductor.
     */
    fun stopForeground() {
        Log.d(logTag, "stopForeground")

        // Shutdown conductor
        runBlocking { runtime!!.stop() }
        runtime = null

        // Stop service
        super.stopForeground(true)
        stopSelf()
    }

    /**
     * Starts the service in the foreground and launches the Holochain conductor.
     *
     * Creates a persistent notification indicating that the Holochain service is running,
     * and starts the Holochain conductor with a default configuration.
     */
    private fun startForeground(config: RuntimeNetworkConfigFfi) {
        // Create the notification to display while the service is running
        val notification =
            NotificationCompat
                .Builder(this, NOTIFICATION_CHANNEL_ID_FOREGROUND_SERVICE)
                .setContentTitle("Holochain Service")
                .setContentText("Holochain Service is running")
                .setSmallIcon(R.drawable.holochain_logo)
                .setOngoing(true)
                .build()

        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) {
            this.startForeground(NOTIFICATION_ID_FOREGROUND_SERVICE_RUNNING, notification)
        } else {
            this.startForeground(NOTIFICATION_ID_FOREGROUND_SERVICE_RUNNING, notification, FOREGROUND_SERVICE_TYPE_DATA_SYNC)
        }

        // Passphrase is currently *hard-coded*
        // See https://github.com/holochain/android-service-runtime/issues/11
        val passphrase = byteArrayOf(0x48, 101, 108, 108, 111)

        // Start holochain conductor
        serviceScope.launch(Dispatchers.IO) {
            runtime =
                RuntimeFfi.start(
                    passphrase,
                    RuntimeConfigFfi(getFilesDir().toString(), config),
                )
            Log.d(logTag, "Holochain started successfully")
        }
    }

    /**
     * Gets or creates an AppBinder for a specific Holochain app.
     *
     * Maintains a map of app IDs to binders to avoid creating duplicate binders.
     *
     * @param installedAppId The ID of the Holochain app to bind to
     * @return An AppBinder for the specified app
     */
    private fun getOrCreateAppBinder(installedAppId: String): AppBinder {
        if (!this.appBinders.containsKey(installedAppId)) {
            this.appBinders[installedAppId] = AppBinder(installedAppId)
        }
        return this.appBinders[installedAppId]!!
    }

    /**
     * Gets the AutostartConfigManager, initializing it if necessary.
     *
     * The AutostartConfigManager handles persistence of autostart settings and
     * determines whether the service should start automatically on device boot.
     *
     * @return The AutostartConfigManagerFfi instance
     */
    private fun getAutostartConfigManager(): AutostartConfigManagerFfi {
        if (this.autostartConfigManager == null) {
            this.autostartConfigManager = AutostartConfigManagerFfi(getFilesDir().toString())
        }

        return this.autostartConfigManager as AutostartConfigManagerFfi
    }

    /**
     * Checks if the POST_NOTIFICATIONS permission has been granted.
     *
     * On Android 13 (API 33) and above, the app must request this permission at runtime.
     * On earlier Android versions, this permission was granted automatically.
     *
     * @return true if the permission is granted or not required, false otherwise
     */
    private fun checkNotificationPermission(): Boolean {
        Log.d(logTag, "checkNotificationPermission")

        // On Android 13 (API 33) and above, your app must request `POST_NOTIFICATIONS` permission at runtime.
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            return true
        }

        return ContextCompat.checkSelfPermission(
            this,
            android.Manifest.permission.POST_NOTIFICATIONS,
        ) == PackageManager.PERMISSION_GRANTED
    }

    /**
     * Shows a notification asking the user to authorize an app's access to a Holochain app.
     *
     * Creates a notification with approve and deny actions, and stores the request in the
     * authorization request map for later retrieval.
     *
     * @param clientPackageName The package name of the Android app requesting access
     * @param installedAppId The ID of the Holochain app being accessed
     * @return The UUID of the stored authorization request
     */
    private fun showAppAuthorizationNotification(
        clientPackageName: String,
        installedAppId: String,
    ): String {
        Log.d(
            logTag,
            "showAppAuthorizationNotification clientPackageName=$clientPackageName installedAppId=$installedAppId",
        )

        // Store app authorization request
        val putResponse = appAuthorizationRequests.put(AppAuthorizationRequest(clientPackageName, installedAppId))

        // Create action intents, including id of app authorization request
        // so we can retrieve it later
        val approveIntent =
            Intent(this, HolochainService::class.java).apply {
                action = HolochainServiceIntentActions.ACTION_APPROVE_APP_AUTHORIZATION
                putExtra("requestId", putResponse.uuid)
            }
        val approvePendingIntent =
            PendingIntent.getService(
                this,
                2,
                approveIntent,
                PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
            )

        val denyIntent =
            Intent(this, HolochainService::class.java).apply {
                action = HolochainServiceIntentActions.ACTION_DENY_APP_AUTHORIZATION
                putExtra("requestId", putResponse.uuid)
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
                .Builder(this, NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION)
                .setContentTitle("Authorize App Connection")
                .setStyle(
                    NotificationCompat
                        .BigTextStyle()
                        .bigText(
                            "The app \"$clientPackageName\" wants to connect to the Holochain app: \"$installedAppId\".\n\nDo you want to allow this connection?",
                        ),
                ).setSmallIcon(R.drawable.holochain_logo)
                .setPriority(NotificationCompat.PRIORITY_HIGH)
                .setAutoCancel(true)
                .addAction(R.drawable.ic_baseline_check, "Approve", approvePendingIntent)
                .addAction(R.drawable.ic_outline_cancel, "Deny", denyPendingIntent)
                .build()

        // Show the notification
        if (checkNotificationPermission()) {
            NotificationManagerCompat.from(this).notify(NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION, putResponse.notificationId, notification)
        } else {
            Log.w(logTag, "Cannot show authorization notification: POST_NOTIFICATIONS permission not granted")
        }

        return putResponse.uuid
    }

    private inner class AdminBinder : IHolochainServiceAdmin.Stub() {
        private val logTag = "IHolochainServiceAdmin"
        private var authorized = false
        private var clientPackageName: String? = null

        // Stop the conductor
        //
        // Because this stops the service,
        // we cannot call back over IPC.
        //
        // So instead it is a no-op if not authorized
        override fun stop() {
            Log.d(logTag, "stop")

            if (!this.isAuthorized()) {
                return
            }

            // Disable conductor autostart on system boot
            this@HolochainService.getAutostartConfigManager().disable()

            // Stop conductor and service
            this@HolochainService.stopForeground()
        }

        // Is the conductor started and ready to receive calls
        // Allowed to be called without authorization
        override fun isReady(): Boolean {
            Log.d(logTag, "isReady")

            return runtime != null
        }

        // / Setup an app
        override fun setupApp(
            callback: IHolochainServiceCallback,
            payload: InstallAppPayloadFfiParcel,
            enableAfterInstall: Boolean,
        ) {
            Log.d(logTag, "setupApp")
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

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
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

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
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

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
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

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
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

            serviceScope.launch(Dispatchers.IO) {
                runtime!!.disableApp(installedAppId)
                callback.disableApp()
            }
        }

        // / List installed apps
        override fun listApps(callback: IHolochainServiceCallback) {
            Log.d(logTag, "listApps")
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

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
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

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
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

            serviceScope.launch(Dispatchers.IO) {
                callback.ensureAppWebsocket(AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!))
            }
        }

        // / Sign a zome call
        override fun signZomeCall(
            callback: IHolochainServiceCallback,
            req: ZomeCallParamsFfiParcel,
        ) {
            Log.d(logTag, "signZomeCall")
            if (!this.isAuthorized()) {
                callbackUnauthorized(callback)
                return
            }

            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallParamsSignedFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }

        // We cannot call Binder.getCallingUid() within the onBind callback,
        // so instead we check the authorization within each IPC call
        private fun loadClientPackageName() {
            if (this.clientPackageName != null) {
                return
            }

            val clientUid = Binder.getCallingUid()
            this.clientPackageName =
                packageManager.getNameForUid(clientUid)
                    ?: throw AppBinderUnauthorizedException(
                        "Unable to get name of package binding to AdminBinder uid=$clientUid",
                    )
        }

        private fun loadAuthorized() {
            if (this.authorized) {
                return
            }

            this.authorized = clientPackageName == HolochainService@ packageName
        }

        private fun isAuthorized(): Boolean {
            this.loadClientPackageName()
            this.loadAuthorized()
            return this.authorized
        }

        private fun callbackUnauthorized(callback: IHolochainServiceCallback) {
            Log.d(logTag, "AdminBinder expectAuthorized")

            if (!isAuthorized()) {
                callback.adminBinderUnauthorizedException(
                    AdminBinderUnauthorizedExceptionParcel(
                        AdminBinderUnauthorizedException(
                            "Package is not authorized to access the AdminBinder",
                        ),
                    ),
                )
            }
        }
    }

    private inner class AppBinder(
        private val installedAppId: String,
    ) : IHolochainServiceApp.Stub() {
        private val logTag = "IHolochainServiceApp installedAppId=$installedAppId"
        private var authorized = false
        private var clientPackageName: String? = null

        // / Setup an app
        override fun setupApp(
            callback: IHolochainServiceCallback,
            payload: InstallAppPayloadFfiParcel,
            enableAfterInstall: Boolean,
        ) {
            Log.d(logTag, "setupApp")
            if (!this.isAuthorized()) {
                requestUserAuthorization(callback)
                return
            }

            if (payload.inner.installedAppId != installedAppId) {
                throw AppBinderUnauthorizedException(
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
            if (!this.isAuthorized()) {
                requestUserAuthorization(callback)
                return
            }

            serviceScope.launch(Dispatchers.IO) {
                callback.enableApp(AppInfoFfiParcel(runtime!!.enableApp(installedAppId)))
            }
        }

        // / Get or create an app websocket with an authenticated token
        override fun ensureAppWebsocket(callback: IHolochainServiceCallback) {
            Log.d(logTag, "ensureAppWebsocket")
            if (!this.isAuthorized()) {
                requestUserAuthorization(callback)
                return
            }

            serviceScope.launch(Dispatchers.IO) {
                callback.ensureAppWebsocket(AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!))
            }
        }

        // / Sign a zome call
        override fun signZomeCall(
            callback: IHolochainServiceCallback,
            req: ZomeCallParamsFfiParcel,
        ) {
            Log.d(logTag, "signZomeCall")
            if (!this.isAuthorized()) {
                requestUserAuthorization(callback)
                return
            }

            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallParamsSignedFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }

        // We cannot call Binder.getCallingUid() within the onBind callback,
        // so instead we check the authorization within each IPC call
        private fun loadClientPackageName() {
            if (this.clientPackageName != null) {
                return
            }

            val clientUid = Binder.getCallingUid()
            this.clientPackageName =
                packageManager.getNameForUid(clientUid)
                    ?: throw AppBinderUnauthorizedException(
                        "Unable to get name of package binding to AdminBinder uid=$clientUid",
                    )
        }

        private fun loadAuthorized() {
            if (this.authorized) {
                return
            }
            this.authorized = runtime!!.isAppClientAuthorized(this.clientPackageName!!, this.installedAppId)
        }

        private fun isAuthorized(): Boolean {
            this.loadClientPackageName()
            this.loadAuthorized()
            return this.authorized
        }

        private fun requestUserAuthorization(callback: IHolochainServiceCallback) {
            if (this.isAuthorized()) {
                return
            }

            // Show notification asking user to authorize this app client
            HolochainService@ showAppAuthorizationNotification(this.clientPackageName!!, this.installedAppId)

            callback.appBinderUnauthorizedException(
                AppBinderUnauthorizedExceptionParcel(
                    AppBinderUnauthorizedException(
                        "Package is not authorized to access the AppBinder",
                    ),
                ),
            )
        }
    }
}
