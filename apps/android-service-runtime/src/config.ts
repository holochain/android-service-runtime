import { type RuntimeNetworkConfig } from "tauri-plugin-holochain-service-api";

/**
 * Network Config to apply to the runtime.
 * 
 * Note that peers MUST have the same `bootstrapUrl` and `signalUrl` to communicate
 * with each other.
 * 
 * However, peers MAY have different `iceUrls`.
 * 
 * Therefore, the `bootstrapUrl` and `signalUrl` should be configured by the 
 * distributor of this app, not by the users themselves.
 */
export const RUNTIME_NETWORK_CONFIG: RuntimeNetworkConfig = {
    bootstrapUrl: "https://bootstrap-0.infra.holochain.org",
    signalUrl: "wss://sbd.holo.host",
    iceUrls: ["stun:stun.l.google.com:19302"]
};