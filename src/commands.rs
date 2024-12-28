use fabric::FabricGameVersion;
use fabric::FabricLoaderVersion;
use forge::ForgeVersionList;
use quilt::QuiltGameVersion;
use quilt::QuiltLoaderVersion;
use tauri::{command, AppHandle, Runtime};
use vanilla::VanillaGameVersion;

use crate::models::*;
use crate::MinecraftLauncherExt;
use crate::Result;

#[command]
pub(crate) async fn get_vanilla_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<VanillaGameVersion>> {
    app.minecraft_launcher().get_vanilla_versions().await
}

#[command]
pub(crate) async fn get_fabric_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<FabricGameVersion>> {
    app.minecraft_launcher().get_fabric_versions().await
}

#[command]
pub(crate) async fn get_fabric_loaders<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<FabricLoaderVersion>> {
    app.minecraft_launcher().get_fabric_loaders().await
}

#[command]
pub(crate) async fn get_quilt_versions<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<QuiltGameVersion>> {
    app.minecraft_launcher().get_quilt_versions().await
}

#[command]
pub(crate) async fn get_quilt_loaders<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Vec<QuiltLoaderVersion>> {
    app.minecraft_launcher().get_quilt_loaders().await
}

#[command]
pub(crate) async fn get_forge_metadata<R: Runtime>(app: AppHandle<R>) -> Result<ForgeVersionList> {
    app.minecraft_launcher().get_forge_metadata().await
}
