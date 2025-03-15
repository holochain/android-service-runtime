package com.plugin.holochain_service_consumer

import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceClient
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.toParcel

@TauriPlugin
class HolochainServicePlugin(private val activity: Activity): Plugin(activity) {
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private lateinit var serviceClient: HolochainServiceClient

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.injectholochainclientenv)
        this.injectHolochainClientEnvJavascript = resourceInputStream.bufferedReader().use { it.readText() }
    }

    /**
     * Start the service
     */
    @Command
    fun connect(invoke: Invoke) {
        this.serviceClient = HolochainServiceClient(
            this.activity,
            "com.plugin.holochain_service.HolochainService",
            "org.holochain.androidserviceruntime.app"
        )
        this.serviceClient.connect()
        invoke.resolve()
    }

    /**
     * Install an app
     */
    @Command
    fun installApp(invoke: Invoke) {
        val args = invoke.parseArgs(InstallAppPayloadFfiInvokeArg::class.java)
        this.serviceClient.installApp(args.toFfi().toParcel())
        invoke.resolve()
    }

    /**
     * Is an app with the given app_id installed
     */
    @Command
    fun isAppInstalled(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        val res = this.serviceClient.isAppInstalled(args.installedAppId)
        val obj = JSObject()
        obj.put("installed", res)
        invoke.resolve(obj)
    }

    /**
     * Get or create an app websocket with authentication token
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    @Command
    fun ensureAppWebsocket(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        val res = this.serviceClient.ensureAppWebsocket(args.installedAppId)

        val obj = JSObject()
        obj.put("ensureAppWebsocket", res.inner.toJSObject())
        invoke.resolve(obj)
    }

    /**
     * Sign Zome Call
     */
    @Command
    fun signZomeCall(invoke: Invoke) {
        val args = invoke.parseArgs(ZomeCallUnsignedFfiInvokeArg::class.java)
        val res = this.serviceClient.signZomeCall(ZomeCallUnsignedFfiParcel(args.toFfi()))
        invoke.resolve(res.inner.toJSObject())
    }
}
