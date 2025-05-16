package org.holochain.androidserviceruntime.plugin.service

import app.tauri.annotation.InvokeArg
import org.holochain.androidserviceruntime.client.CellIdFfi
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfi
import org.holochain.androidserviceruntime.client.RoleSettingsFfi
import org.holochain.androidserviceruntime.client.ZomeCallUnsignedFfi

@InvokeArg
class InstallAppPayloadFfiInvokeArg {
    lateinit var source: ByteArray
    lateinit var installedAppId: String
    lateinit var networkSeed: String
    lateinit var roleSettings: Map<String, RoleSettingsFfi>
}

fun InstallAppPayloadFfiInvokeArg.toFfi(): InstallAppPayloadFfi =
    InstallAppPayloadFfi(
        this.source,
        this.installedAppId,
        this.networkSeed,
        this.roleSettings,
    )

@InvokeArg
class AppIdInvokeArg {
    lateinit var installedAppId: String
}

@InvokeArg
class ZomeCallUnsignedFfiInvokeArg {
    lateinit var provenance: ByteArray
    lateinit var cellId: CellIdFfi
    lateinit var zomeName: String
    lateinit var fnName: String
    var capSecret: ByteArray? = null
    lateinit var payload: ByteArray
    lateinit var nonce: ByteArray
    var expiresAt: Long = 0L
}

fun ZomeCallUnsignedFfiInvokeArg.toFfi(): ZomeCallUnsignedFfi =
    ZomeCallUnsignedFfi(
        this.provenance,
        this.cellId,
        this.zomeName,
        this.fnName,
        this.capSecret,
        this.payload,
        this.nonce,
        this.expiresAt,
    )

@InvokeArg
class RuntimeNetworkConfigFfiInvokeArg {
    lateinit var bootstrap_url: String
    lateinit var signal_url: String
    lateinit var ice_urls: List<String>
}

fun RuntimeNetworkConfigFfiInvokeArg.toFfi(): RuntimeNetworkConfigFfi =
    RuntimeNetworkConfigFfi(
        this.bootstrap_url,
        this.signal_url,
        this.ice_urls
    )