package org.holochain.androidserviceruntime.service

class UnauthorizedException(
    message: String = "Unauthorized",
    cause: Throwable? = null,
) : Exception(message, cause)
