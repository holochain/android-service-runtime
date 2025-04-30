package org.holochain.androidserviceruntime.plugin.service

import android.app.Activity
import android.app.NotificationManager
import android.app.NotificationChannel
import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.util.Log
import android.webkit.WebView
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import org.holochain.androidserviceruntime.client.HolochainServiceAdminClient
import org.holochain.androidserviceruntime.client.toJSONArrayString
import org.holochain.androidserviceruntime.client.toJSONObjectString
import org.holochain.androidserviceruntime.service.HolochainService

@TauriPlugin
class HolochainServicePlugin(
    private val activity: Activity,
) : Plugin(activity) {
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private val serviceComponentName = ComponentName(this.activity, HolochainService::class.java)
    private var serviceClient = HolochainServiceAdminClient(this.activity, this.serviceComponentName)
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val logTag = "HolochainServicePlugin"

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        Log.d(logTag, "load")
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.holochainenv)
        this.injectHolochainClientEnvJavascript = resourceInputStream.bufferedReader().use { it.readText() }

        // Create notification channels
        val notificationManager = activity.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.createNotificationChannel(
            NotificationChannel(
                HolochainService.NOTIFICATION_CHANNEL_ID_FOREGROUND_SERVICE,
                "Running Status",
                NotificationManager.IMPORTANCE_HIGH,
            ),
        )
        notificationManager.createNotificationChannel(
            NotificationChannel(
                HolochainService.NOTIFICATION_CHANNEL_ID_APP_AUTHORIZATION,
                "Authorization Requests",
                NotificationManager.IMPORTANCE_HIGH,
            ),
        )
        // Attempt to connect to service
        // It may not be running
        this.serviceClient.connect()
    }
    
    /**
     * Check if POST_NOTIFICATIONS permission has been granted
     */
    private fun hasPostNotificationsPermission(): Boolean {
        Log.d(logTag, "hasPostNotificationsPermission")

        // Before Tiramisu, POST_NOTIFICATIONS permission does not need
        // to be granted manually at runtime.
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.TIRAMISU) {
            return true;
        }

        return ContextCompat.checkSelfPermission(
            this.activity,
            android.Manifest.permission.POST_NOTIFICATIONS
        ) == PackageManager.PERMISSION_GRANTED
    }
    
    /**
     * Request POST_NOTIFICATIONS permission
     */
    private fun requestPostNotificationsPermission() {
        Log.d(logTag, "requestPostNotificationsPermission")

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            if (!this.hasPostNotificationsPermission()) {
                Log.d(logTag, "Lacking post notifications permission")
                ActivityCompat.requestPermissions(this.activity, arrayOf(android.Manifest.permission.POST_NOTIFICATIONS), 123)
            }
        }
    }

    /**
     * Start the service
     */
    @Command
    fun start(invoke: Invoke) {
        Log.d(logTag, "start")

        // Request POST_NOTIFICATION permission, so the HolochainService can create notifications
        this.requestPostNotificationsPermission()

        // Start service
        val intent = Intent()
        intent.setComponent(this.serviceComponentName)
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
        Log.d(logTag, "stop")
        this.serviceClient.stop()
        invoke.resolve()
    }

    /**
     * Is service ready to receive calls
     */
    @Command
    fun isReady(invoke: Invoke) {
        Log.d(logTag, "isReady")
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
        Log.d(logTag, "installApp")
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
        Log.d(logTag, "isAppInstalled")
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
        Log.d(logTag, "uninstallApp")
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
        Log.d(logTag, "enableApp")
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
        Log.d(logTag, "disableApp")
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
        Log.d(logTag, "listApps")
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
        Log.d(logTag, "ensureAppWebsocket")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            val res = serviceClient.ensureAppWebsocket(args.installedAppId)

            // Inject launcher env into web view
            injectHolochainClientEnv(
                args.installedAppId,
                res.port.toInt(),
                res.authentication.token.toUByteArray(),
            )

            invoke.resolve(JSObject(res.toJSONObjectString()))
        }
    }

    /**
     * Sign Zome Call
     */
    @Command
    fun signZomeCall(invoke: Invoke) {
        Log.d(logTag, "signZomeCall")
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
    private fun injectHolochainClientEnv(
        appId: String,
        appWebsocketPort: Int,
        appWebsocketToken: UByteArray,
    ) {
        Log.d(logTag, "injectHolochainClientEnv")
        // Declare js helper function for injecting holochain client env, bundled with dependencies
        this.webView.evaluateJavascript(this.injectHolochainClientEnvJavascript, null)

        // Inject holochain client env
        val tokenJsArray = appWebsocketToken.toMutableList().toJSONArrayString()
        this.webView.evaluateJavascript(
            """window.injectHolochainClientEnv("$appId", $appWebsocketPort, $tokenJsArray) """,
            null,
        )
    }
}
