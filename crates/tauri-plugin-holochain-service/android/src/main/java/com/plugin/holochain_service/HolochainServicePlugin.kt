package com.plugin.holochain_service

import android.app.Activity
import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
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

        // Create notification channel
        val notificationManager = activity.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.createNotificationChannel(NotificationChannel(
            "HolochainServiceChannel",
            "Holochain Service Running",
            NotificationManager.IMPORTANCE_HIGH
        ))
    }

    /**
     * Start the service
     */
    @Command
    fun start(invoke: Invoke) {
        this.serviceClient = HolochainServiceClient(this.activity)
        this.serviceClient.connect()
        invoke.resolve()
    }
    
    /**
     * Shutdown the service
     */
    @Command
    fun stop(invoke: Invoke) {
        this.serviceClient.stop()
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
     * Uninstall an app
     */
    @Command
    fun uninstallApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        this.serviceClient.uninstallApp(args.installedAppId)
        invoke.resolve()
    }

    /**
     * Enable an app
     */
    @Command
    fun enableApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        this.serviceClient.enableApp(args.installedAppId)
        invoke.resolve()
    }

    /**
     * Disable an app
     */
    @Command
    fun disableApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        this.serviceClient.disableApp(args.installedAppId)
        invoke.resolve()
    }

    /**
     * List installed apps
     */
    @Command
    fun listApps(invoke: Invoke) {
        val res = this.serviceClient.listApps()
        val obj = JSObject() 
        obj.put("installedApps", res.toJSArray())
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

        // Inject launcher env into web view
        this.injectHolochainClientEnv(
            args.installedAppId,
            res.inner.port.toInt(),
            res.inner.authentication.token.toUByteArray()
        )
        
        val obj = JSObject() 
        obj.put("ensureAppWebsocket", res.toJSObject())
        invoke.resolve(obj)       
    }

    /**
     * Sign Zome Call
     */
    @Command
    fun signZomeCall(invoke: Invoke) {
        val args = invoke.parseArgs(ZomeCallUnsignedFfiInvokeArg::class.java)
        val res = this.serviceClient.signZomeCall(ZomeCallUnsignedFfiParcel(args.toFfi()))
        invoke.resolve(res.toJSObject())
    }

    /**
     * Inject magic holochain-client-js variables into webview window
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    private fun injectHolochainClientEnv(appId: String, appWebsocketPort: Int, appWebsocketToken: UByteArray) {
        // Declare js helper function for injecting holochain client env, bundled with dependencies
        this.webView.evaluateJavascript(this.injectHolochainClientEnvJavascript, null)

        // Inject holochain client env
        val tokenJsArray = appWebsocketToken.toMutableList().toJSArray() 
        this.webView.evaluateJavascript(
            """injectHolochainClientEnv("$appId", ${appWebsocketPort}, ${tokenJsArray}) """,
            null
        )
    }
}
