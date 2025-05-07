package org.holochain.androidserviceruntime.service

import java.util.concurrent.atomic.AtomicInteger

public class NotificationIdGenerator(
    private val initialValue: Int = 0,
) {
    private var counter = AtomicInteger(initialValue)

    public fun next(): Int = counter.getAndIncrement()
}
