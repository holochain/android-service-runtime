package com.plugin.holochain_service_consumer

import android.app.Activity
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
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceClient
import org.holochain.androidserviceruntime.holochain_service_client.toJSONObjectString

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

    /**
     * Start the service
     */
    @Command
    fun connect(invoke: Invoke) {
        Log.d(TAG, "connect")
        this.serviceClient.connect()
        invoke.resolve()
    }

    /**
     * Install an app
     */
    @Command
    fun installApp(invoke: Invoke) {
        Log.d(TAG, "installApp")
        val args = invoke.parseArgs(InstallAppPayloadFfiInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            val res = serviceClient.installApp(args.toFfi())
            invoke.resolve(JSObject(res.toJSONObjectString()))
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
     * Get or create an app websocket with authentication token
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    @Command
    fun ensureAppWebsocket(invoke: Invoke) {
        Log.d(TAG, "ensureAppWebsocket")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            val res = serviceClient.ensureAppWebsocket(args.installedAppId)
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
}
