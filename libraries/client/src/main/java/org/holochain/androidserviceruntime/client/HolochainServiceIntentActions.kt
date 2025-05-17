package org.holochain.androidserviceruntime.client

object HolochainServiceIntentActions {
    // Start the Service, and the Holochain conductor, if autostart on boot is enabled
    const val ACTION_BOOT_COMPLETED = "org.holochain.androidserviceruntime.service.ACTION_BOOT_COMPLETED"

    // Approve app authorization request
    const val ACTION_APPROVE_APP_AUTHORIZATION = "org.holochain.androidserviceruntime.service.APPROVE_APP_AUTHORIZATION"

    // Deny app authorization request
    const val ACTION_DENY_APP_AUTHORIZATION = "org.holochain.androidserviceruntime.service.DENY_APP_AUTHORIZATION"

    // Start the Service, and the Holochain conductor
    const val ACTION_START = "org.holochain.androidserviceruntime.service.ACTION_START"
}
