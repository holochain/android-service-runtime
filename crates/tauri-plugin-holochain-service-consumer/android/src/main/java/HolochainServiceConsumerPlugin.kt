package com.plugin.holochain_service_consumer

import android.app.Activity
import android.content.Intent
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.view.Gravity
import android.view.LayoutInflater
import android.view.ViewGroup
import android.view.WindowManager
import android.webkit.WebView
import android.util.Log
import android.view.View
import android.widget.Button
import android.widget.FrameLayout
import android.widget.TextView
import androidx.cardview.widget.CardView
import androidx.constraintlayout.widget.ConstraintLayout
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
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceNotConnectedException

@TauriPlugin
class HolochainServiceConsumerPlugin(private val activity: Activity): Plugin(activity) {
    private lateinit var webView: WebView
    private var overlayView: FrameLayout? = null
    private var notificationCard: CardView? = null
    private lateinit var injectHolochainClientEnvJavascript: String
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val servicePackage = "org.holochain.androidserviceruntime.app"
    private var serviceClient = HolochainServiceClient(
        this.activity,
        servicePackage,
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

    /**
     * Creates and shows a semi-transparent overlay that blocks the UI
     */
    private fun showBlurOverlay() {
        activity.runOnUiThread {
            try {
                // Remove any existing overlay first
                removeBlurOverlay()
                
                // Get the root view of the activity
                val rootView = activity.findViewById<ViewGroup>(android.R.id.content)
                
                // Create a new FrameLayout for the overlay
                overlayView = FrameLayout(activity)
                overlayView?.layoutParams = FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT
                )
                
                // Set a semi-transparent background
                overlayView?.background = ColorDrawable(Color.parseColor("#80000000")) // 50% transparent black
                
                // Block touch events by setting an OnClickListener
                overlayView?.isClickable = true
                overlayView?.isFocusable = true
                overlayView?.setOnClickListener { 
                    // Consume the click event and do nothing
                    Log.d(TAG, "Overlay clicked - preventing interaction with elements behind")
                }
                
                // Add the overlay to the root view with highest z-index
                rootView.addView(overlayView)
                
                Log.d(TAG, "Blur overlay added with touch blocking")
            } catch (e: Exception) {
                Log.e(TAG, "Failed to show blur overlay", e)
            }
        }
    }
    
    /**
     * Removes the blur overlay and notification
     */
    private fun removeBlurOverlay() {
        activity.runOnUiThread {
            try {
                // Clear notification reference
                notificationCard = null
                
                // Remove overlay
                overlayView?.let {
                    val rootView = activity.findViewById<ViewGroup>(android.R.id.content)
                    rootView.removeView(it)
                    overlayView = null
                    Log.d(TAG, "Blur overlay and notification removed")
                }
            } catch (e: Exception) {
                Log.e(TAG, "Failed to remove blur overlay", e)
            }
        }
    }

    /**
     * Shows a centered notification that Holochain Service is not running
     */
    private fun showServiceNotConnectedNotice() {
        Log.d(TAG, "showServiceNotConnectedNotice")
        try {
            // Show blur overlay and custom notification on UI thread
            activity.runOnUiThread {
                // Add blur overlay first
                showBlurOverlay()
                
                // Remove any existing notification
                if (notificationCard != null) {
                    (overlayView as ViewGroup).removeView(notificationCard)
                    notificationCard = null
                }
                
                // Inflate custom notification layout
                val inflater = LayoutInflater.from(activity)
                val notificationView = inflater.inflate(R.layout.custom_notification, null)
                notificationCard = notificationView as CardView
                
                // Set background color based on theme
                val isNightMode = activity.resources.configuration.uiMode and 
                    android.content.res.Configuration.UI_MODE_NIGHT_MASK == 
                    android.content.res.Configuration.UI_MODE_NIGHT_YES
                
                val bgColor = if (isNightMode) {
                    activity.resources.getColor(R.color.notification_background_dark, activity.theme)
                } else {
                    activity.resources.getColor(R.color.notification_background_light, activity.theme)
                }
                
                val textColor = if (isNightMode) {
                    activity.resources.getColor(R.color.notification_text_dark, activity.theme)
                } else {
                    activity.resources.getColor(R.color.notification_text_light, activity.theme)
                }
                
                val buttonBgColor = if (isNightMode) {
                    activity.resources.getColor(R.color.button_background_dark, activity.theme)
                } else {
                    activity.resources.getColor(R.color.button_background_light, activity.theme)
                }
                
                // Set colors based on theme
                notificationCard?.setCardBackgroundColor(bgColor)
                val titleText = notificationView.findViewById<TextView>(R.id.notificationTitle)
                val messageText = notificationView.findViewById<TextView>(R.id.notificationMessage)
                val actionButton = notificationView.findViewById<Button>(R.id.notificationAction)
                
                titleText.setTextColor(textColor)
                messageText.setTextColor(textColor)
                actionButton.setBackgroundColor(buttonBgColor)
                
                // Set up the action button
                actionButton.setOnClickListener {
                    try {
                        // Launch Holochain service app
                        val launchIntent = activity.packageManager.getLaunchIntentForPackage(servicePackage)
                        if (launchIntent != null) {
                            activity.startActivity(launchIntent)
                            // Remove overlay when user navigates away
                            removeBlurOverlay()
                        } else {
                            Log.e(TAG, "Could not find launch intent for Holochain Service app")
                        }
                    } catch (e: Exception) {
                        Log.e(TAG, "Failed to launch Holochain Service app", e)
                    }
                }
                
                // Add the notification to the overlay with centered layout params
                val layoutParams = FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT
                )
                layoutParams.gravity = Gravity.CENTER
                layoutParams.leftMargin = 48
                layoutParams.rightMargin = 48
                
                overlayView?.addView(notificationCard, layoutParams)
            }
        } catch (e: Exception) {
            // Log failure but don't crash
            Log.e(TAG, "Failed to show notification", e)
        }
    }
    }
}
