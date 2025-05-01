package org.holochain.androidserviceruntime.client

class AdminBinderUnauthorizedException(
    message: String = "AdminBinderUnauthorized",
) : Exception(message, null)

class AppBinderUnauthorizedException(
    message: String = "AppBinderUnauthorized",
) : Exception(message, null)
