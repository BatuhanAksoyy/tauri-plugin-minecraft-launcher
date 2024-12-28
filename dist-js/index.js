import { invoke } from '@tauri-apps/api/core';

async function getVanillaVersions() {
    return await invoke('plugin:minecraft-launcher|get_vanilla_versions');
}
async function getFabricVersions() {
    return await invoke('plugin:minecraft-launcher|get_fabric_versions');
}
async function getFabricLoaders() {
    return await invoke('plugin:minecraft-launcher|get_fabric_loaders');
}
async function getQuiltVersions() {
    return await invoke('plugin:minecraft-launcher|get_quilt_versions');
}
async function getQuiltLoaders() {
    return await invoke('plugin:minecraft-launcher|get_quilt_loaders');
}
async function getForgeVersions() {
    return await invoke('plugin:minecraft-launcher|get_forge_versions');
}
async function getForgeLoaders() {
    return await invoke('plugin:minecraft-launcher|get_forge_loaders');
}

export { getFabricLoaders, getFabricVersions, getForgeLoaders, getForgeVersions, getQuiltLoaders, getQuiltVersions, getVanillaVersions };
