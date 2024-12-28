use fabric::{FabricGameVersion, FabricLoaderVersion, FabricVersionList};
use forge::ForgeVersionList;
use lyceris::http;
use quilt::{QuiltGameVersion, QuiltLoaderVersion, QuiltVersionList};
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};
use vanilla::{VanillaGameVersion, VanillaVersionList};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MinecraftLauncher<R>> {
    Ok(MinecraftLauncher(app.clone()))
}

/// Access to the minecraft-launcher APIs.
pub struct MinecraftLauncher<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_vanilla_versions(&self) -> crate::Result<Vec<VanillaGameVersion>> {
        Ok(http::fetch::fetch::<VanillaVersionList>(
            "https://launchermeta.mojang.com/mc/game/version_manifest.json",
        )
        .await?
        .versions)
    }
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_fabric_versions(&self) -> crate::Result<Vec<FabricGameVersion>> {
        Ok(
            http::fetch::fetch::<FabricVersionList>("https://meta.fabricmc.net/v2/versions")
                .await?
                .game,
        )
    }
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_fabric_loaders(&self) -> crate::Result<Vec<FabricLoaderVersion>> {
        Ok(
            http::fetch::fetch::<FabricVersionList>("https://meta.fabricmc.net/v2/versions")
                .await?
                .loader,
        )
    }
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_quilt_versions(&self) -> crate::Result<Vec<QuiltGameVersion>> {
        Ok(
            http::fetch::fetch::<QuiltVersionList>("https://meta.fabricmc.net/v2/versions")
                .await?
                .game,
        )
    }
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_quilt_loaders(&self) -> crate::Result<Vec<QuiltLoaderVersion>> {
        Ok(
            http::fetch::fetch::<QuiltVersionList>("https://meta.fabricmc.net/v2/versions")
                .await?
                .loader,
        )
    }
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_forge_loaders(&self) -> crate::Result<Vec<String>> {
        Ok(http::fetch::fetch::<ForgeVersionList>(
            "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json",
        )
        .await?
        .values()
        .flat_map(|list| {
            list.iter()
                .filter_map(|version| version.split("-").nth(1).map(|s| s.to_string()))
        })
        .collect::<Vec<String>>())
    }
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn list_forge_versions(&self) -> crate::Result<Vec<String>> {
        Ok(http::fetch::fetch::<ForgeVersionList>(
            "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json",
        )
        .await?
        .values()
        .flat_map(|list| {
            list.iter()
                .filter_map(|version| version.split("-").nth(0).map(|s| s.to_string()))
        })
        .collect::<Vec<String>>())
    }
}
