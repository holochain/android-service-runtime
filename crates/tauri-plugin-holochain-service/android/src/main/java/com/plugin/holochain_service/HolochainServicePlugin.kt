package com.plugin.holochain_service

import android.app.Activity
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.webkit.WebView
import android.util.Log
import kotlinx.coroutines.launch
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceAdminClient
import org.holochain.androidserviceruntime.holochain_service_client.toJSONObjectString
import org.holochain.androidserviceruntime.holochain_service_client.toJSONArrayString

@TauriPlugin
class HolochainServicePlugin(private val activity: Activity): Plugin(activity) {
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private val packageName = "org.holochain.androidserviceruntime.app"
    private val className = "com.plugin.holochain_service.HolochainService"
    private var serviceClient = HolochainServiceAdminClient(
        this.activity,
        this.packageName,
        this.className,
    )
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val TAG = "HolochainServicePlugin"

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

        // Create notification channel
        val notificationManager = activity.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.createNotificationChannel(NotificationChannel(
            "HolochainServiceChannel",
            "Holochain Service Running",
            NotificationManager.IMPORTANCE_HIGH
        ))
        
        // Attempt to connect to service
        // It may not be running
        this.serviceClient.connect()
    }

    /**
     * Start the service
     */
    @Command
    fun start(invoke: Invoke) {
        Log.d(TAG, "start")

        // Start service
        val intent = Intent()
        intent.setComponent(ComponentName(this.packageName, this.className))
        this.activity.startForegroundService(intent)

        // Connect to service
        this.serviceClient.connect()
        invoke.resolve()
    }

    /**
     * Shutdown the service
     */
    @Command
    fun stop(invoke: Invoke) {
        Log.d(TAG, "stop")
        this.serviceClient.stop()
        invoke.resolve()
    }

    /**
     * Is service ready to receive calls
     */
    @Command
    fun isReady(invoke: Invoke) {
        Log.d(TAG, "isReady")
        val res = this.serviceClient.isReady()
        val obj = JSObject()
        obj.put("ready", res)
        invoke.resolve(obj)
    }

    /**
     * Install an app
     */
    @Command
    fun installApp(invoke: Invoke) {
        Log.d(TAG, "installApp")
        val args = invoke.parseArgs(InstallAppPayloadFfiInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            serviceClient.installApp(args.toFfi())
            invoke.resolve()
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
            val res = serviceClient.isAppInstalled(args.installedAppId)
            val obj = JSObject()
            obj.put("installed", res)
            invoke.resolve(obj)
        }
    }

    /**
     * Uninstall an app
     */
    @Command
    fun uninstallApp(invoke: Invoke) {
        Log.d(TAG, "uninstallApp")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            serviceClient.uninstallApp(args.installedAppId)
            invoke.resolve()
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
            val res = serviceClient.enableApp(args.installedAppId)
            invoke.resolve(JSObject(res.toJSONObjectString()))
        }
    }

    /**
     * Disable an app
     */
    @Command
    fun disableApp(invoke: Invoke) {
        Log.d(TAG, "disableApp")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            serviceClient.disableApp(args.installedAppId)
            invoke.resolve()
        }
    }

    /**
     * List installed apps
     */
    @Command
    fun listApps(invoke: Invoke) {
        Log.d(TAG, "listApps")
        serviceScope.launch(Dispatchers.IO) {
            val res = serviceClient.listApps()
            val obj = JSObject()
            obj.put("installedApps", JSArray(res.toJSONArrayString()))
            invoke.resolve(obj)
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
            val res = serviceClient.ensureAppWebsocket(args.installedAppId)

            // Inject launcher env into web view
            injectHolochainClientEnv(
                args.installedAppId,
                res.port.toInt(),
                res.authentication.token.toUByteArray()
            )

            invoke.resolve(JSObject(res.toJSONObjectString()))
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
            val res = serviceClient.signZomeCall(args.toFfi())
            invoke.resolve(JSObject(res.toJSONObjectString()))
        }
    }

    /**
     * Inject magic holochain-client-js variables into webview window
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    private fun injectHolochainClientEnv(appId: String, appWebsocketPort: Int, appWebsocketToken: UByteArray) {
        Log.d(TAG, "injectHolochainClientEnv")
        // Declare js helper function for injecting holochain client env, bundled with dependencies
        this.webView.evaluateJavascript(this.injectHolochainClientEnvJavascript, null)

        // Inject holochain client env
        val tokenJsArray = appWebsocketToken.toMutableList().toJSONArrayString()
        this.webView.evaluateJavascript(
            """window.injectHolochainClientEnv("$appId", ${appWebsocketPort}, ${tokenJsArray}) """,
            null
        )
    }
}
