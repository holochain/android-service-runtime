package org.holochain.androidserviceruntime.holochain_service_client

class HolochainServiceNotConnectedException(
    message: String = "Holochain service is not connected",
    cause: Throwable? = null,
) : Exception(message, cause)
