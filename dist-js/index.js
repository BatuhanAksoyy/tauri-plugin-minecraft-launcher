import { invoke } from '@tauri-apps/api/core';

async function listVanillaVersions() {
    return await invoke('plugin:minecraft-launcher|list_vanilla_versions');
}
async function listFabricVersions() {
    return await invoke('plugin:minecraft-launcher|list_fabric_versions');
}
async function listFabricLoaders() {
    return await invoke('plugin:minecraft-launcher|list_fabric_loaders');
}
async function listQuiltVersions() {
    return await invoke('plugin:minecraft-launcher|list_quilt_versions');
}
async function listQuiltLoaders() {
    return await invoke('plugin:minecraft-launcher|list_quilt_loaders');
}
async function listForgeVersions() {
    return await invoke('plugin:minecraft-launcher|list_forge_versions');
}
async function listForgeLoaders() {
    return await invoke('plugin:minecraft-launcher|list_forge_loaders');
}

export { listFabricLoaders, listFabricVersions, listForgeLoaders, listForgeVersions, listQuiltLoaders, listQuiltVersions, listVanillaVersions };
