package com.plugin.holochain_service_consumer

import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceClientApp
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.toParcel

@TauriPlugin
class HolochainServiceConsumerPlugin(private val activity: Activity): Plugin(activity) {
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private lateinit var serviceClient: HolochainServiceClient
    private lateinit var installedAppId: String

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
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        this.installedAppId = args.installedAppId
        this.serviceClient = HolochainServiceClient(
            this.activity,
            args.installedAppId,
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
        val args = invoke.parseArgs(ConsumerInstallAppPayloadFfiInvokeArg::class.java)
        this.serviceClient.installApp(args.toFfi(this.installedAppId).toParcel())
        invoke.resolve()
    }

    /**
     * Enable an app
     */
    @Command
    fun enableApp(invoke: Invoke) {
        var res = this.serviceClient.enableApp(this.installedAppId)
        val obj = JSObject()
        obj.put("enabled", res)
        invoke.resolve(obj)
    }

    /**
     * Is an app with the given app_id installed
     */
    @Command
    fun isAppInstalled(invoke: Invoke) {
        val res = this.serviceClient.isAppInstalled(this.installedAppId)
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
        val res = this.serviceClient.ensureAppWebsocket(this.installedAppId)

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
