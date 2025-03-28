package com.plugin.holochain_service

import android.util.Log
import android.app.Service
import android.os.IBinder
import android.os.Binder
import android.content.Intent
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.DelicateCoroutinesApi
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
import org.holochain.androidserviceruntime.holochain_service_client.fromParcel

class HolochainService : Service() {
    /// The uniffi-generated holochain runtime bindings
    public var runtime: RuntimeFfi? = null

    private val TAG = "HolochainService"
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)

    /// The IPC receiver that other activities can call into
    @OptIn(DelicateCoroutinesApi::class, ExperimentalUnsignedTypes::class)
    private val binderAdmin = object : IHolochainServiceAdmin.Stub() {
        private val TAG = "IHolochainServiceAdmin"

        /// Stop the conductor
        override fun stop() {
            Log.d(TAG, "shutdown")
            stopForeground()
        }

        /// Is the conductor started and ready to receive calls
        override fun isReady(): Boolean {
            Log.d(TAG, "isReady")
            return runtime != null
        }
        
        /// Install an app
        override fun installApp(
            callback: IHolochainServiceCallback,
            request: InstallAppPayloadFfiParcel
        ) {
            Log.d(TAG, "installApp")
            serviceScope.launch(Dispatchers.IO) {
                callback.installApp(AppInfoFfiParcel(runtime!!.installApp(request.fromParcel())))
            }
        }

        /// Uninstall an app
        override fun uninstallApp(
            callback: IHolochainServiceCallback,
            installedAppId: String
        ) {
            Log.d(TAG, "uninstallApp")
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
            serviceScope.launch(Dispatchers.IO) {
                runtime!!.disableApp(installedAppId)
                callback.disableApp()
            }
        }

        /// List installed apps
        override fun listApps(callback: IHolochainServiceCallback) {
            Log.d(TAG, "listApps")
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
            serviceScope.launch(Dispatchers.IO) {
                callback.signZomeCall(ZomeCallFfiParcel(runtime!!.signZomeCall(req.inner)))
            }
        }
    }

    fun createBinderApp(installedAppId: String): IBinder {
        return object : IHolochainServiceApp.Stub() {
            private val TAG = "IHolochainServiceApp"

            /// Install an app
            override fun installApp(
                callback: IHolochainServiceCallback,
                request: InstallAppPayloadFfiParcel
            ) {
                if(request.installedAppId == installedAppId) {  
                    Log.d(TAG, "installApp")
                    serviceScope.launch(Dispatchers.IO) {
                        callback.installApp(AppInfoFfiParcel(runtime!!.installApp(request.fromParcel())))
                    }
                } else {
                    // TODO throw error
                }
            }

            /// Enable an installed app
            override fun enableApp(
                callback: IHolochainServiceCallback,
            ) {
                Log.d(TAG, "enableApp")
                serviceScope.launch(Dispatchers.IO) {
                    callback.enableApp(AppInfoFfiParcel(runtime!!.enableApp(installedAppId)))
                }
            }

            /// Is app installed
            override fun isAppInstalled(
                callback: IHolochainServiceCallback,
            ) {
                Log.d(TAG, "isAppInstalled")
                serviceScope.launch(Dispatchers.IO) {
                    callback.isAppInstalled(runtime!!.isAppInstalled(installedAppId))
                }
            }

            /// Get or create an app websocket with an authenticated token
            override fun ensureAppWebsocket(
                callback: IHolochainServiceCallback,
            ) {
                Log.d(TAG, "ensureAppWebsocket")
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
                serviceScope.launch(Dispatchers.IO) {
                    callback.signZomeCall(ZomeCallFfiParcel(runtime!!.signZomeCall(req.inner)))
                }
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
        // Get the name of the package that is attempting to bind
        var callingUid: Int = Binder.getCallingUid()
        var clientPackageName: String = getPackageManager().getPackagesForUid(callingUid)!![0]

        // Which api are they attempting to bind to?
        var api = intent.getStringExtra("api")

        Log.d(TAG, "onBind: api=" + api + " clientPackageName=" + clientPackageName)

        if (api == "admin") {
            // Only this package can call the admin api
            if(clientPackageName == this.getPackageName()) {
                return binderAdmin
            } else {
                // TODO throw error, unauthorized admin api request
            }
        } else if (api == "app") {
            var installedAppId = intent.getStringExtra("installedAppId")!!
            var isAuthorized = runtime!!.isAppClientAuthorized(
                clientPackageName,
                installedAppId
            )

            if(isAuthorized) {
                return this.createBinderApp(installedAppId)
            } else {
                // TODO notify user to request authorization

                // Authorize app client
                runtime!!.authorizeAppClient(
                    clientPackageName,
                    installedAppId,
                )
                return this.createBinderApp(installedAppId) 
            }
        } else {
            // TODO Throw error, invalid api
        }
        
        return null
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
