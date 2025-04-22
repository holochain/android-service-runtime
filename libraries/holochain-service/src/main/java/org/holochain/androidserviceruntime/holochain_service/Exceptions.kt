package org.holochain.androidserviceruntime.holochain_service

class UnauthorizedException(
    message: String = "Unauthorized",
    cause: Throwable? = null,
) : Exception(message, cause)
