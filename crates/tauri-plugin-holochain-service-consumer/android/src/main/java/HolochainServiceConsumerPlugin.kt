package com.plugin.holochain_service_consumer

import android.app.Activity
import android.content.Intent
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.view.Gravity
import android.view.LayoutInflater
import android.view.ViewGroup
import android.webkit.WebView
import android.util.Log
import android.widget.Button
import android.widget.FrameLayout
import android.widget.TextView
import androidx.cardview.widget.CardView
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
    private lateinit var holochainEnvJs: String
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val servicePackage = "org.holochain.androidserviceruntime.app"
    private var serviceClient = HolochainServiceClient(
        this.activity,
        servicePackage,
        "com.plugin.holochain_service.HolochainService"
    )
    private var TAG = "HolochainServiceConsumerPlugin"
    private var overlayView: FrameLayout? = null
    private var notificationCard: CardView? = null

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        Log.d(TAG, "load")
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.holochainenv)
        this.holochainEnvJs = resourceInputStream.bufferedReader().use { it.readText() }
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
            try {
                val res = serviceClient.setupApp(args.toInstallAppPayloadFfi(), args.enableAfterInstall)
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                handleCommandException(e, invoke)
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
                handleCommandException(e, invoke)
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
                handleCommandException(e, invoke)
            }
        }
    }

    /**
     * Display the service notice if the exception is HolochainServiceNotConnectedException
     */
    private fun handleCommandException(e: Exception, invoke: Invoke) {
        if (e is HolochainServiceNotConnectedException) {
            showServiceNotConnectedNotice()
            invoke.reject(e.toString(), "HolochainServiceNotConnected")
        }
        else {
            invoke.reject(e.toString())
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
                // Create Blur Background View
                overlayView = FrameLayout(activity)
                overlayView!!.layoutParams = FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT
                )

                // 50% transparent black
                overlayView!!.background = ColorDrawable(Color.parseColor("#80000000"))

                // Prevent clicking
                overlayView!!.isClickable = true
                overlayView!!.isFocusable = true
                overlayView!!.setOnClickListener { }


                // Create Notice View
                val inflater = LayoutInflater.from(activity)
                val notificationView = inflater.inflate(R.layout.custom_notification, null)
                notificationCard = notificationView as CardView

                // Set colors
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
                notificationCard?.setCardBackgroundColor(bgColor)
                notificationView.findViewById<TextView>(R.id.notificationTitle)
                    .setTextColor(textColor)
                notificationView.findViewById<TextView>(R.id.notificationMessage)
                    .setTextColor(textColor)
                val openSettingsActionButton = notificationView.findViewById<Button>(R.id.openSettingsAction)
                val reloadActionButton = notificationView.findViewById<Button>(R.id.reloadAction)
                openSettingsActionButton.setBackgroundColor(buttonBgColor)
                reloadActionButton.setBackgroundColor(buttonBgColor)

                // Set layout
                val layoutParams = FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT
                )
                layoutParams.gravity = Gravity.CENTER
                layoutParams.leftMargin = 48
                layoutParams.rightMargin = 48


                // Button Callbacks
                openSettingsActionButton.setOnClickListener {
                    try {
                        // Launch Holochain service app
                        val launchIntent = activity.packageManager.getLaunchIntentForPackage(servicePackage)
                        if (launchIntent != null) {
                            activity.startActivity(launchIntent)
                        } else {
                            Log.e(TAG, "Could not find launch intent for Holochain Service app")
                        }
                    } catch (e: Exception) {
                        Log.e(TAG, "Failed to launch Holochain Service app", e)
                    }
                }
                reloadActionButton.setOnClickListener {
                    removeBlurOverlay();
                    this.webView.reload()
                }

                // Render views
                val rootView = activity.findViewById<ViewGroup>(android.R.id.content)
                overlayView!!.addView(notificationCard, layoutParams)
                rootView.addView(overlayView)
            }
        } catch (e: Exception) {
            // Log failure but don't crash
            Log.e(TAG, "Failed to show notification", e)
        }
    }
}
