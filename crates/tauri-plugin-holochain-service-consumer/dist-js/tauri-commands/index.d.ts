export declare function installApp(request: {
    appId: string;
    appBundleBytes: Uint8Array;
    membraneProofs: Map<String, Uint8Array>;
    agent?: Uint8Array;
    networkSeed: String;
}): Promise<null>;
export declare function isAppInstalled(appId: string): Promise<boolean>;
export declare function ensureAppWebsocket(appId: string): Promise<{
    appId: string;
    port: number;
    token: Uint8Array;
} | null>;
