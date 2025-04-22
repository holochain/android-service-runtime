package com.plugin.holochain_service_consumer

import android.app.Activity
import android.webkit.WebView
import android.util.Log
import android.content.ComponentName
import kotlinx.coroutines.launch
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceAppClient
import org.holochain.androidserviceruntime.holochain_service_client.toJSONObjectString
import org.holochain.androidserviceruntime.holochain_service_client.HolochainServiceNotConnectedException

@TauriPlugin
class HolochainServiceConsumerPlugin(private val activity: Activity): Plugin(activity) {
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val servicePackage = "org.holochain.androidserviceruntime.app"
    private val serviceClient = HolochainServiceAppClient(
        this.activity,
        ComponentName(servicePackage, "org.holochain.androidserviceruntime.holochain_service.HolochainService")
    )
    private val disconnectedNotice = DisconnectedNotice(activity, servicePackage)
    private val TAG = "HolochainServiceConsumerPlugin"
    private var webView: WebView? = null

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        Log.d(TAG, "load")
        super.load(webView)
        this.webView = webView

        disconnectedNotice.load()
    }

    /**
     * Connect to service, wait for connection to be ready, then setup an app
     */
    @Command
    fun connectSetupApp(invoke: Invoke) {
        Log.d(TAG, "connectSetupApp")
        val args = invoke.parseArgs(SetupAppConfigInvokeArg::class.java)        
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.connectSetupApp(args.toInstallAppPayloadFfi(), args.enableAfterInstall)
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
        Log.d(TAG, "handleCommandException")
        if (e is HolochainServiceNotConnectedException) {
            if(this.webView == null) {
                disconnectedNotice.enableShowOnLoad()
            } else {
                disconnectedNotice.show()
            }
            invoke.reject(e.toString(), "HolochainServiceNotConnected")
        }
        else {
            invoke.reject(e.toString())
        }
    }
}
