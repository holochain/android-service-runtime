package com.plugin.holochain_service_consumer

import app.tauri.annotation.InvokeArg
import org.holochain.androidserviceruntime.holochain_service_client.CellIdFfi
import org.holochain.androidserviceruntime.holochain_service_client.InstallAppPayloadFfi
import org.holochain.androidserviceruntime.holochain_service_client.RoleSettingsFfi
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfi

@InvokeArg
class SetupAppInvokeArg {
    lateinit var source: ByteArray
    lateinit var installedAppId: String
    lateinit var networkSeed: String
    lateinit var roleSettings: Map<String, RoleSettingsFfi>
    var enableAfterInstall: Boolean = true
}

fun SetupAppInvokeArg.toInstallAppPayloadFfi(): InstallAppPayloadFfi {
    return InstallAppPayloadFfi(
        this.source,
        this.installedAppId,
        this.networkSeed,
        this.roleSettings,
    )
}

@InvokeArg
class AppIdInvokeArg {
    lateinit var installedAppId: String
}

@InvokeArg
class ZomeCallUnsignedFfiInvokeArg {
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

fun ZomeCallUnsignedFfiInvokeArg.toFfi(): ZomeCallUnsignedFfi {
    return ZomeCallUnsignedFfi(
        this.provenance,
        CellIdFfi(
            dnaHash = this.cellIdDnaHash,
            agentPubKey = this.cellIdAgentPubKey
        ),
        this.zomeName,
        this.fnName,
        this.capSecret,
        this.payload,
        this.nonce,
        this.expiresAt
    )
}
