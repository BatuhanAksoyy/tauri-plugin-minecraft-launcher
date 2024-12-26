use fabric::FabricGameVersion;
use fabric::FabricLoaderVersion;
use quilt::QuiltGameVersion;
use quilt::QuiltLoaderVersion;
use tauri::{command, AppHandle, Runtime};
use vanilla::VanillaGameVersion;

use crate::models::*;
use crate::MinecraftLauncherExt;
use crate::Result;

#[command]
pub(crate) async fn list_vanilla_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<VanillaGameVersion>> {
    app.minecraft_launcher().list_vanilla_versions().await
}

#[command]
pub(crate) async fn list_fabric_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<FabricGameVersion>> {
    app.minecraft_launcher().list_fabric_versions().await
}

#[command]
pub(crate) async fn list_fabric_loaders<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<FabricLoaderVersion>> {
    app.minecraft_launcher().list_fabric_loaders().await
}

#[command]
pub(crate) async fn list_quilt_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<QuiltGameVersion>> {
    app.minecraft_launcher().list_quilt_versions().await
}

#[command]
pub(crate) async fn list_quilt_loaders<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<QuiltLoaderVersion>> {
    app.minecraft_launcher().list_quilt_loaders().await
}

#[command]
pub(crate) async fn list_forge_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<String>> {
    app.minecraft_launcher().list_forge_versions().await
}

#[command]
pub(crate) async fn list_forge_loaders<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<String>> {
    app.minecraft_launcher().list_forge_loaders().await
}