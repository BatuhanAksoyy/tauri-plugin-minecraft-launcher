export interface VanillaGameVersion {
    id: string;
    type: string;
    url: string;
    time: string;
    releaseTime: string;
}
export interface FabricGameVersion {
    version: string;
    stable: boolean;
}
export interface FabricLoaderVersion {
    separator: string;
    build: number;
    maven: string;
    version: string;
    stable: boolean;
}
export interface QuiltGameVersion {
    version: string;
    stable: boolean;
}
export interface QuiltLoaderVersion {
    separator: string;
    build: number;
    maven: string;
    version: string;
}
export interface Config {
    gameDir: string;
    version: string;
    authentication: {
        Offline: {
            username: string;
            uuid: string;
        };
    } | {
        Microsoft: {
            access_token: string;
            refresh_token: string;
            uuid: string;
            xuid: string;
            username: string;
        };
    };
    memory: {
        Gigabyte: number;
    } | {
        Megabyte: number;
    };
    versionName: string;
    profile: Profile;
    loader: LoaderConfig;
    javaVersion: string;
    runtimeDir: string;
    customJavaArgs: string[];
    customArgs: string[];
}
export interface Profile {
    name: string;
    root: string;
}
export interface LoaderConfig {
    type: string;
    version: string;
}
export type ForgeVersionList = Map<string, string[]>;
export declare function getVanillaVersions(): Promise<VanillaGameVersion[]>;
export declare function getFabricVersions(): Promise<FabricGameVersion[]>;
export declare function getFabricLoaders(): Promise<FabricLoaderVersion[]>;
export declare function getQuiltVersions(): Promise<QuiltGameVersion[]>;
export declare function getQuiltLoaders(): Promise<QuiltLoaderVersion[]>;
export declare function getForgeMetadata(): Promise<ForgeVersionList>;
export declare function installMinecraft(config: Config): Promise<void>;
export declare function launchMinecraft(config: Config): Promise<void>;
