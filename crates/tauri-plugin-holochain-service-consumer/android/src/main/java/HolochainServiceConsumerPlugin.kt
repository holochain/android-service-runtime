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
        
        // The notification channel is now created in showServiceNotRunningNotification
        // to ensure it has all the right settings every time
    }

    /**
     * Show a notification that Holochain Service is not running
     */
    private fun showServiceNotRunningNotification() {
        Log.d(TAG, "showServiceNotRunningNotification")
        try {
            val notificationManager = activity.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            
            // Ensure channel exists with proper settings
            val channel = NotificationChannel(
                "HolochainServiceErrorChannel",
                "Holochain Service Errors",
                NotificationManager.IMPORTANCE_HIGH
            )
            channel.description = "Notifications for Holochain Service errors"
            channel.enableLights(true)
            channel.enableVibration(true)
            notificationManager.createNotificationChannel(channel)
            
            // Build notification with icon from Android system
            val notification = NotificationCompat.Builder(activity, "HolochainServiceErrorChannel")
                .setContentTitle("Holochain Service Error")
                .setContentText("Holochain Service is not running")
                .setSmallIcon(android.R.drawable.ic_dialog_alert)
                .setPriority(NotificationCompat.PRIORITY_HIGH)
                .setAutoCancel(true)
                .build()
                
            notificationManager.notify(100, notification)
            Log.d(TAG, "Notification shown for service error")
            
            // Show toast on UI thread
            activity.runOnUiThread {
                Toast.makeText(activity, "Holochain Service is not running", Toast.LENGTH_LONG).show()
            }
        } catch (e: Exception) {
            // Log failure but don't crash
            Log.e(TAG, "Failed to show notification", e)
        }
    }
    
    /**
     * Handle service connection errors
     */
    private fun handleServiceError(invoke: Invoke, error: Throwable, errorMessage: String) {
        Log.e(TAG, errorMessage, error)
        if (error is IllegalStateException && error.message == "Service not connected") {
            Log.d(TAG, "SERVICE NOT CONNECTED")
            showServiceNotRunningNotification()
            val errorObj = JSObject()
            errorObj.put("error", "SERVICE_NOT_CONNECTED")
            errorObj.put("message", "Holochain Service is not running")
            invoke.reject(errorMessage, errorObj)
        } else {
            Log.d(TAG, "other error")
            invoke.reject(errorMessage, error as Exception)
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
            handleServiceError(invoke, e, "Failed to connect to Holochain Service")
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
                handleServiceError(invoke, e, "Failed to install app")
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
            val res = serviceClient.enableApp(args.installedAppId)
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
            try {
                val res = serviceClient.isAppInstalled(args.installedAppId)
                val obj = JSObject()
                obj.put("installed", res)
                invoke.resolve(obj)
            } catch (e: Exception) {
                handleServiceError(invoke, e, "Failed to check if app is installed")
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
                handleServiceError(invoke, e, "Failed to ensure app websocket")
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
                handleServiceError(invoke, e, "Failed to sign zome call")
            }
        }
    }
}
