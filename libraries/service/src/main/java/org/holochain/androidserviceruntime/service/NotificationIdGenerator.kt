package org.holochain.androidserviceruntime.service

import java.util.concurrent.atomic.AtomicInteger

/**
 * Generates sequential, thread-safe notification IDs.
 *
 * Used to create unique IDs for Android notifications.
 *
 * @param initialValue The starting value for the notification IDs (default: 0)
 */
public class NotificationIdGenerator(
    private val initialValue: Int = 0,
) {
    private var counter = AtomicInteger(initialValue)

    /**
     * Gets the next available notification ID.
     *
     * Thread-safe operation that returns a unique ID and increments the counter.
     *
     * @return A unique notification ID
     */
    public fun next(): Int = counter.getAndIncrement()
}
