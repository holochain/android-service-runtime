package com.plugin.holochain_service_consumer

import app.tauri.annotation.InvokeArg

@InvokeArg
class InstallAppRequestArgs {
    lateinit var appId: String
    lateinit var appBundleBytes: ByteArray
    lateinit var membraneProofs: Map<String, ByteArray>
    var agent: ByteArray? = null
    var networkSeed: String? = null
}

@InvokeArg
class AppIdRequestArgs {
    lateinit var appId: String
}

@InvokeArg
class SignZomeCallRequestArgs {
    lateinit var provenance: ByteArray
    lateinit var cellIdDnaHash: ByteArray
    lateinit var cellIdAgentPubKey: ByteArray
    lateinit var zomeName: String
    lateinit var fnName: String
    var capSecret: ByteArray? = null
    lateinit var payload: ByteArray
    lateinit var nonce: ByteArray
    var expiresAt: Long = 0L
}
