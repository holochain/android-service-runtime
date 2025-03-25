package com.plugin.holochain_service

import android.util.Log
import android.app.Service
import android.os.IBinder
import android.content.Intent
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import com.plugin.holochain_service.holochain_conductor_runtime_ffi.RuntimeFfi
import org.holochain.androidserviceruntime.holochain_service_client.IHolochainService
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
    private val binder = object : IHolochainService.Stub() {
        private val TAG = "IHolochainService"

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
            request: InstallAppPayloadFfiParcel
        ): AppInfoFfiParcel {
            Log.d(TAG, "installApp")
            return runBlocking {
                AppInfoFfiParcel(runtime!!.installApp(request.fromParcel()))
            }
        }

        /// Uninstall an app
        override fun uninstallApp(
            installedAppId: String
        ) {
            Log.d(TAG, "uninstallApp")
            return runBlocking {
                runtime!!.uninstallApp(installedAppId)
            }
        }

        /// Enable an installed app
        override fun enableApp(
            installedAppId: String
        ): AppInfoFfiParcel {
            Log.d(TAG, "enableApp")
            return runBlocking {
                AppInfoFfiParcel(runtime!!.enableApp(installedAppId))
            }
        }

        /// Disable an installed app
        override fun disableApp(
            installedAppId: String
        ) {
            Log.d(TAG, "disableApp")
            return runBlocking {
                runtime!!.disableApp(installedAppId)
            }
        }

        /// List installed apps
        override fun listApps(callback: IHolochainServiceCallback) {
            Log.d(TAG, "listApps")
            serviceScope.launch(Dispatchers.Default) {
                callback.listApps(runtime!!.listApps().map {
                    AppInfoFfiParcel(it)
                })
            }
        }

        /// Is app installed
        override fun isAppInstalled(installedAppId: String): Boolean {
            Log.d(TAG, "isAppInstalled")
            return runBlocking {
                runtime!!.isAppInstalled(installedAppId)
            }
        }

        /// Get or create an app websocket with an authenticated token
        override fun ensureAppWebsocket(installedAppId: String): AppAuthFfiParcel {
            Log.d("IHolochainService", "ensureAppWebsocket")
            return runBlocking {
                AppAuthFfiParcel(runtime?.ensureAppWebsocket(installedAppId)!!)
            }
        }

        /// Sign a zome call
        override fun signZomeCall(req: ZomeCallUnsignedFfiParcel): ZomeCallFfiParcel {
            Log.d("IHolochainService", "signZomeCall")
            return runBlocking {
                val res = runtime!!.signZomeCall(req.inner)
                ZomeCallFfiParcel(res)
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

    override fun onBind(intent: Intent?): IBinder = binder

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
            
            serviceScope.launch(Dispatchers.Default) {
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
