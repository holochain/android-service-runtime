export interface CellId {
    agentPubKey: Uint8Array;
    dnaHash: Uint8Array;
}
export interface DnaModifiers {
    networkSeed: string;
    properties: Uint8Array;
}
export interface CellInfoV1 {
    cellId: CellId;
    dnaModifiers: DnaModifiers;
    name: string;
}
export interface CellInfo {
    v1: CellInfoV1;
}
export interface AppInfo {
    agentPubKey: Uint8Array;
    cellInfo: Map<string, CellInfo>;
}
export type RuntimeNetworkConfig = {
    bootstrapUrl: string;
    signalUrl: string;
    iceUrls: string[];
};
export declare function setConfig(request: RuntimeNetworkConfig): Promise<null>;
export declare function start(): Promise<string | null>;
export declare function stop(): Promise<string | null>;
export declare function isReady(): Promise<boolean>;
export declare function installApp(request: {
    appId: string;
    appBundleBytes: Uint8Array;
    membraneProofs: Map<String, Uint8Array>;
    agent?: Uint8Array;
    networkSeed: String;
}): Promise<string | null>;
export declare function uninstallApp(installedAppId: string): Promise<null>;
export declare function enableApp(installedAppId: string): Promise<AppInfo>;
export declare function disableApp(installedAppId: string): Promise<null>;
export declare function listApps(): Promise<AppInfo[]>;
export declare function isAppInstalled(installedAppId: string): Promise<boolean>;
export declare function ensureAppWebsocket(installedAppId: string): Promise<{
    port: number;
    authentication: {
        token: Uint8Array;
        expiresAt?: number;
    };
} | null>;
