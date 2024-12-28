'use strict';

var core = require('@tauri-apps/api/core');

async function getVanillaVersions() {
    return await core.invoke('plugin:minecraft-launcher|get_vanilla_versions');
}
async function getFabricVersions() {
    return await core.invoke('plugin:minecraft-launcher|get_fabric_versions');
}
async function getFabricLoaders() {
    return await core.invoke('plugin:minecraft-launcher|get_fabric_loaders');
}
async function getQuiltVersions() {
    return await core.invoke('plugin:minecraft-launcher|get_quilt_versions');
}
async function getQuiltLoaders() {
    return await core.invoke('plugin:minecraft-launcher|get_quilt_loaders');
}
async function getForgeMetadata() {
    return await core.invoke('plugin:minecraft-launcher|get_forge_metadata');
}
async function installMinecraft(version, path, auth) {
    return await core.invoke('plugin:minecraft-launcher|install_minecraft', { version, path, auth });
}

exports.getFabricLoaders = getFabricLoaders;
exports.getFabricVersions = getFabricVersions;
exports.getForgeMetadata = getForgeMetadata;
exports.getQuiltLoaders = getQuiltLoaders;
exports.getQuiltVersions = getQuiltVersions;
exports.getVanillaVersions = getVanillaVersions;
exports.installMinecraft = installMinecraft;
