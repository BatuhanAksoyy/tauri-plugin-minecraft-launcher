'use strict';

var core = require('@tauri-apps/api/core');

async function listVanillaVersions() {
    return await core.invoke('plugin:minecraft-launcher|list_vanilla_versions');
}
async function listFabricVersions() {
    return await core.invoke('plugin:minecraft-launcher|list_fabric_versions');
}
async function listFabricLoaders() {
    return await core.invoke('plugin:minecraft-launcher|list_fabric_loaders');
}
async function listQuiltVersions() {
    return await core.invoke('plugin:minecraft-launcher|list_quilt_versions');
}
async function listQuiltLoaders() {
    return await core.invoke('plugin:minecraft-launcher|list_quilt_loaders');
}
async function listForgeVersions() {
    return await core.invoke('plugin:minecraft-launcher|list_forge_versions');
}
async function listForgeLoaders() {
    return await core.invoke('plugin:minecraft-launcher|list_forge_loaders');
}

exports.listFabricLoaders = listFabricLoaders;
exports.listFabricVersions = listFabricVersions;
exports.listForgeLoaders = listForgeLoaders;
exports.listForgeVersions = listForgeVersions;
exports.listQuiltLoaders = listQuiltLoaders;
exports.listQuiltVersions = listQuiltVersions;
exports.listVanillaVersions = listVanillaVersions;
