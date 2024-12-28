export interface VanillaGameVersion {
    id: string;
    type: string;
    url: string;
    time: string;
    release_time: string;
    sha1: string;
    compliance_level: number;
}
export interface FabricGameVersion {
    version: string;
    stable: boolean;
}
export interface FabricLoaderVersion {
    seperator: string;
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
    seperator: string;
    build: number;
    maven: string;
    version: string;
    stable: boolean;
}
export declare function getVanillaVersions(): Promise<VanillaGameVersion[]>;
export declare function getFabricVersions(): Promise<FabricGameVersion[]>;
export declare function getFabricLoaders(): Promise<FabricLoaderVersion[]>;
export declare function getQuiltVersions(): Promise<QuiltGameVersion[]>;
export declare function getQuiltLoaders(): Promise<QuiltLoaderVersion[]>;
export declare function getForgeVersions(): Promise<string[]>;
export declare function getForgeLoaders(): Promise<string[]>;
