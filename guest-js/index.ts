import { invoke } from '@tauri-apps/api/core'

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
      username: string,
      uuid: string
    },
  } | {
    Microsoft: {
      access_token: string,
      refresh_token: string,
      uuid: string,
      xuid: string,
      username: string
    },
  }
  memory: { Gigabyte: number } | { Megabyte: number };
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

export async function getVanillaVersions(): Promise<VanillaGameVersion[]> {
  return await invoke('plugin:minecraft-launcher|get_vanilla_versions');
}

export async function getFabricVersions(): Promise<FabricGameVersion[]> {
  return await invoke('plugin:minecraft-launcher|get_fabric_versions');
}

export async function getFabricLoaders(): Promise<FabricLoaderVersion[]> {
  return await invoke('plugin:minecraft-launcher|get_fabric_loaders');
}

export async function getQuiltVersions(): Promise<QuiltGameVersion[]> {
  return await invoke('plugin:minecraft-launcher|get_quilt_versions');
}

export async function getQuiltLoaders(): Promise<QuiltLoaderVersion[]> {
  return await invoke('plugin:minecraft-launcher|get_quilt_loaders');
}

export async function getForgeMetadata(): Promise<ForgeVersionList> {
  return await invoke('plugin:minecraft-launcher|get_forge_metadata');
}

export async function installMinecraft(config: Config): Promise<void> {
  return await invoke('plugin:minecraft-launcher|install_minecraft', { config });
}

export async function launchMinecraft(config: Config): Promise<void> {
  return await invoke('plugin:minecraft-launcher|launch_minecraft', { config });
}
