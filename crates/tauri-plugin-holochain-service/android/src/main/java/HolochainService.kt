package com.plugin.holochain_service

import android.util.Log
import android.app.Service
import android.app.ForegroundServiceStartNotAllowedException
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import android.content.Intent
import uniffi.holochain_runtime_uniffi.HolochainRuntimeFfi
import uniffi.holochain_runtime_uniffi.HolochainRuntimeFfiConfig
import uniffi.holochain_runtime_uniffi.CellIdFfi
import uniffi.holochain_runtime_uniffi.ZomeCallUnsignedFfi
import uniffi.holochain_runtime_uniffi.GossipArcClampFfi
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import java.nio.ByteBuffer
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.SupervisorJob

class HolochainService : Service() {
    /// The uniffi-generated holochain runtime bindings
    public var runtime: HolochainRuntimeFfi? = null

    /// Holochain conductor admin websocket port
    public var runtimeAdminWebsocketPort: UShort? = null
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val TAG = "HolochainService"

    /// The IPC receiver that other activities can call into
    @OptIn(DelicateCoroutinesApi::class, ExperimentalUnsignedTypes::class)
    private val binder = object : IHolochainService.Stub() {
        private val TAG = "IHolochainService"

        /// Stop the service
        override fun shutdown() {
            Log.d(TAG, "shutdown")
            stopForeground()
        }
        
        /// Install an app
        override fun installApp(
            request: InstallAppRequestAidl
        ) {
            Log.d(TAG, "installApp")

            // Read appBundle bytes from shared memory
            val appBundleBuffer: ByteBuffer = request.appBundleSharedMemory.mapReadOnly()
            val appBundleBytes: ByteArray = appBundleBuffer.toByteArray()
            
            // Call install app
            serviceScope.launch(Dispatchers.Default) {
                runtime!!.installApp(request.appId, appBundleBytes, request.membraneProofs, request.agent, request.networkSeed)
            }
        }

        /// Uninstall an app
        override fun uninstallApp(
            appId: String
        ) {
            Log.d(TAG, "uninstallApp")
            serviceScope.launch(Dispatchers.Default) {
                runtime!!.uninstallApp(appId)
            }
        }

        /// Enable an installed app
        override fun enableApp(
            appId: String
        ) {
            Log.d(TAG, "enableApp")
            serviceScope.launch(Dispatchers.Default) {
                runtime!!.enableApp(appId)
            }
        }

        /// Disable an installed app
        override fun disableApp(
            appId: String
        ) {
            Log.d(TAG, "disableApp")
            serviceScope.launch(Dispatchers.Default) {
                runtime!!.disableApp(appId)
            }
        }

        /// List installed apps
        override fun listInstalledApps(): List<AppInfoFfiAidl> {
            Log.d(TAG, "listInstalledApps")
            return runBlocking {
                runtime!!.listInstalledApps().map {
                    AppInfoFfiAidl(
                        it.installedAppId,
                        it.cellInfo,
                        AppInfoStatusFfiAidl(it.status::class.simpleName!!),
                        it.agentPubKey
                    )
                }
            }
        }

        /// Is app installed
        override fun isAppInstalled(appId: String): Boolean {
            Log.d(TAG, "isAppInstalled")
            return runBlocking {
                runtime!!.isAppInstalled(appId)
            }
        }

        /// Get or create an app websocket with an authenticated token
        override fun appWebsocketAuth(appId: String): AppWebsocketAuthFfiAidl {
            Log.d("IHolochainService", "appWebsocketAuth")
            return runBlocking {
                val res = runtime?.appWebsocketAuth(appId)!!
                AppWebsocketAuthFfiAidl(res.appId, res.port.toInt(), res.token.toUByteArray())
            }
        }

        /// Sign a zome call
        override fun signZomeCall(request: SignZomeCallRequestAidl): ZomeCallSignedFfiAidl {
            Log.d("IHolochainService", "signZomeCall")
            return runBlocking {
                val res = runtime?.signZomeCall(ZomeCallUnsignedFfi(
                    request.provenance,
                    CellIdFfi(
                        request.cellIdDnaHash,
                        request.cellIdAgentPubKey,
                    ),
                    request.zomeName,
                    request.fnName,
                    request.capSecret,
                    request.payload,
                    request.nonce,
                    request.expiresAt,
                ))!!
                
                ZomeCallSignedFfiAidl(
                    res.cellId.dnaHash,
                    res.cellId.agentPubKey,
                    res.zomeName,
                    res.fnName,
                    res.payload,
                    res.capSecret,
                    res.provenance,
                    res.signature,
                    res.nonce,
                    res.expiresAt,
                )
            }
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground()
        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
        supervisorJob.cancel()
    }

    override fun onBind(intent: Intent?): IBinder = binder

    private fun startForeground() {
        try {
            // Create the notification to display while the service is running
            val notification = NotificationCompat.Builder(this, "HolochainServiceChannel")
                .setContentTitle("Holochain Conductor is Running")
                .build()
            startForeground(1, notification)

            // Start holochain conductor
            val password = byteArrayOf(0x48, 101, 108, 108, 111)
            val config = HolochainRuntimeFfiConfig(
                "https://bootstrap-0.infra.holochain.org",
                "wss://sbd.holo.host",
                getFilesDir().toString(),
                listOf<String>("stun:stun-0.main.infra.holo.host:443", "stun:stun-1.main.infra.holo.host:443"),
                GossipArcClampFfi.FULL,
                true
            );
            this.runtime = runBlocking {
                var r: HolochainRuntimeFfi = HolochainRuntimeFfi.launch(password, config)
                r
            }
            Log.d(TAG, "Holochain started successfully")
        } catch (e: Exception) {
           Log.e(TAG, "Holochain failed to start $e")
        }
        Log.d(TAG, "Admin Port: ${this.runtimeAdminWebsocketPort}")
    }

    fun stopForeground() {
        // Shutdown conductor
        val job = serviceScope.launch(Dispatchers.Default) {
            runtime?.shutdown()
        }

        runBlocking {
            job.join()

            runtime = null
            runtimeAdminWebsocketPort = null

            // Stop service
            super.stopForeground(true)
            stopSelf()
        }
    }
}
