import { type RoleSettings } from '@holochain/client';
export declare function installApp(request: {
    installedAppId: string;
    source: Uint8Array;
    roleSettings: Map<String, RoleSettings>;
    networkSeed: String;
}): Promise<null>;
export declare function isAppInstalled(installedAppId: string): Promise<boolean>;
export declare function ensureAppWebsocket(installedAppId: string): Promise<{
    port: number;
    authentication: {
        token: Uint8Array;
        expiresAt?: number;
    };
} | null>;
