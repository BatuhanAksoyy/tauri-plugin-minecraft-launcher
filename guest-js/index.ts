import { invoke } from '@tauri-apps/api/core'

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

export async function getForgeVersions(): Promise<string[]> {
  return await invoke('plugin:minecraft-launcher|get_forge_versions');
}

export async function getForgeLoaders(): Promise<string[]> {
  return await invoke('plugin:minecraft-launcher|get_forge_loaders');
}


