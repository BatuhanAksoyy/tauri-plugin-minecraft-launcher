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

export async function listVanillaVersions(): Promise<VanillaGameVersion[]> {
  return await invoke('plugin:minecraft-launcher|list_vanilla_versions');
}

export async function listFabricVersions(): Promise<FabricGameVersion[]> {
  return await invoke('plugin:minecraft-launcher|list_fabric_versions');
}

export async function listFabricLoaders(): Promise<FabricLoaderVersion[]> {
  return await invoke('plugin:minecraft-launcher|list_fabric_loaders');
}

export async function listQuiltVersions(): Promise<QuiltGameVersion[]> {
  return await invoke('plugin:minecraft-launcher|list_quilt_versions');
}

export async function listQuiltLoaders(): Promise<QuiltLoaderVersion[]> {
  return await invoke('plugin:minecraft-launcher|list_quilt_loaders');
}

export async function listForgeVersions(): Promise<string[]> {
  return await invoke('plugin:minecraft-launcher|list_forge_versions');
}

export async function listForgeLoaders(): Promise<string[]> {
  return await invoke('plugin:minecraft-launcher|list_forge_loaders');
}


