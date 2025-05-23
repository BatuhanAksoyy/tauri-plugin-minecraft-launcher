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
async function getForgeMetadata() {
    return await invoke('plugin:minecraft-launcher|get_forge_metadata');
}
async function installMinecraft(config) {
    return await invoke('plugin:minecraft-launcher|install_minecraft', { config });
}
async function launchMinecraft(config) {
    return await invoke('plugin:minecraft-launcher|launch_minecraft', { config });
}

export { getFabricLoaders, getFabricVersions, getForgeMetadata, getQuiltLoaders, getQuiltVersions, getVanillaVersions, installMinecraft, launchMinecraft };
