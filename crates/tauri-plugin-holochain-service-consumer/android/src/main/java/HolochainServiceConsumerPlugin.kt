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
     * Setup an app
     */
    @Command
    fun setupApp(invoke: Invoke) {
        Log.d(TAG, "setupApp")
        val args = invoke.parseArgs(SetupAppConfigInvokeArg::class.java)
        Log.d(TAG, "setup app args " + args)
        serviceScope.launch(Dispatchers.IO) {
            val res = serviceClient.setupApp(args.toInstallAppPayloadFfi(), args.enableAfterInstall)
            invoke.resolve(JSObject(res.toJSONObjectString()))
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
