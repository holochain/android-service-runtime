package org.holochain.androidserviceruntime.client

class AdminBinderUnauthorizedException(
    message: String = "Unauthorized",
    cause: Throwable? = null,
) : Exception(message, cause)
