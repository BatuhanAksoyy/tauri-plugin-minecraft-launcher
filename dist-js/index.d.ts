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
export declare function listVanillaVersions(): Promise<VanillaGameVersion[]>;
export declare function listFabricVersions(): Promise<FabricGameVersion[]>;
export declare function listFabricLoaders(): Promise<FabricLoaderVersion[]>;
export declare function listQuiltVersions(): Promise<QuiltGameVersion[]>;
export declare function listQuiltLoaders(): Promise<QuiltLoaderVersion[]>;
export declare function listForgeVersions(): Promise<string[]>;
export declare function listForgeLoaders(): Promise<string[]>;
