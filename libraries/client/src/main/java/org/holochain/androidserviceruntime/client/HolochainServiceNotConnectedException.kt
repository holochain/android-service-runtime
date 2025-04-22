package org.holochain.androidserviceruntime.client

class HolochainServiceNotConnectedException(
    message: String = "Holochain service is not connected",
    cause: Throwable? = null,
) : Exception(message, cause)
