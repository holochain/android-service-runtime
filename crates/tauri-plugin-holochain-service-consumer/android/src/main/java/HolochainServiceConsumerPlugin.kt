package com.plugin.holochain_service_consumer

import android.app.Activity
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.webkit.WebView
import android.util.Log
import android.widget.Toast
import androidx.core.app.NotificationCompat
import kotlinx.coroutines.launch
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceClient
import org.holochain.androidserviceruntime.holochain_service_client.toJSONObjectString
import java.lang.IllegalStateException

@TauriPlugin
class HolochainServiceConsumerPlugin(private val activity: Activity): Plugin(activity) {
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private var serviceClient = HolochainServiceClient(
        this.activity,
        "org.holochain.androidserviceruntime.app",
        "com.plugin.holochain_service.HolochainService"
    )
    private var TAG = "HolochainServiceConsumerPlugin"

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        Log.d(TAG, "load")
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.holochainenv)
        this.injectHolochainClientEnvJavascript = resourceInputStream.bufferedReader().use { it.readText() }
    }

    override fun onNewIntent(intent: Intent) {
        Log.d(TAG, "onNewIntent")
        try {
            this.serviceClient.connect()
        } catch (e: Exception) {
            if (e is HolochainServiceNotConnectedException) {
                showServiceNotConnectedNotice()
            }
        }
    }

    /**
     * Start the service
     */
    @Command
    fun connect(invoke: Invoke) {
        Log.d(TAG, "connect")
        try {
            this.serviceClient.connect()
            invoke.resolve()
        } catch (e: Exception) {
            if (e is HolochainServiceNotConnectedException) {
                showServiceNotConnectedNotice()
            }
            invoke.reject(e.toString())
        }
    }

    /**
     * Install an app
     */
    @Command
    fun installApp(invoke: Invoke) {
        Log.d(TAG, "installApp")
        val args = invoke.parseArgs(InstallAppPayloadFfiInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.installApp(args.toFfi())
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                if (e is HolochainServiceNotConnectedException) {
                    showServiceNotConnectedNotice()
                }
                invoke.reject(e.toString())
            }
        }
    }

    /**
     * Enable an app
     */
    @Command
    fun enableApp(invoke: Invoke) {
        Log.d(TAG, "enableApp")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
            val res = serviceClient.enableApp(args.installedAppId)
            invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                if (e is HolochainServiceNotConnectedException) {
                    showServiceNotConnectedNotice()
                }
                invoke.reject(e.toString())
            }
        }
    }

    /**
     * Is an app with the given app_id installed
     */
    @Command
    fun isAppInstalled(invoke: Invoke) {
        Log.d(TAG, "isAppInstalled")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.isAppInstalled(args.installedAppId)
                val obj = JSObject()
                obj.put("installed", res)
                invoke.resolve(obj)
            } catch (e: Exception) {
                if (e is HolochainServiceNotConnectedException) {
                    showServiceNotConnectedNotice()
                }
                invoke.reject(e.toString())
            }
        }
    }

    /**
     * Get or create an app websocket with authentication token
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    @Command
    fun ensureAppWebsocket(invoke: Invoke) {
        Log.d(TAG, "ensureAppWebsocket")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.ensureAppWebsocket(args.installedAppId)
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                if (e is HolochainServiceNotConnectedException) {
                    showServiceNotConnectedNotice()
                }
                invoke.reject(e.toString())
            }
        }
    }

    /**
     * Sign Zome Call
     */
    @Command
    fun signZomeCall(invoke: Invoke) {
        Log.d(TAG, "signZomeCall")
        val args = invoke.parseArgs(ZomeCallUnsignedFfiInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.signZomeCall(args.toFfi())
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                if (e is HolochainServiceNotConnectedException) {
                    showServiceNotConnectedNotice()
                }
                invoke.reject(e.toString())
            }
        }
    }

            }
        }
    }
}
