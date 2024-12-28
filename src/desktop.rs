use std::{cell::LazyCell, path::PathBuf, sync::LazyLock};

use fabric::{FabricGameVersion, FabricLoaderVersion};
use forge::ForgeVersionList;
use lyceris::{
    auth::AuthMethod,
    http,
    minecraft::{
        config::{Config, ConfigBuilder},
        emitter::Emitter,
        install::install,
        launch::launch,
        loader::Loader,
    },
};
use quilt::{QuiltGameVersion, QuiltLoaderVersion};
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

#[allow(clippy::incompatible_msrv)]
pub static EMITTER: LazyLock<Emitter> = LazyLock::new(Emitter::default);

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn get_vanilla_versions(&self) -> crate::Result<Vec<VanillaGameVersion>> {
        Ok(http::fetch::fetch::<VanillaVersionList>(
            "https://launchermeta.mojang.com/mc/game/version_manifest.json",
        )
        .await?
        .versions)
    }

    pub async fn get_fabric_versions(&self) -> crate::Result<Vec<FabricGameVersion>> {
        Ok(http::fetch::fetch("https://meta.fabricmc.net/v2/versions/game").await?)
    }

    pub async fn get_fabric_loaders(&self) -> crate::Result<Vec<FabricLoaderVersion>> {
        Ok(http::fetch::fetch("https://meta.fabricmc.net/v2/versions/loader").await?)
    }

    pub async fn get_quilt_versions(&self) -> crate::Result<Vec<QuiltGameVersion>> {
        Ok(http::fetch::fetch("https://meta.quiltmc.org/v3/versions/game").await?)
    }

    pub async fn get_quilt_loaders(&self) -> crate::Result<Vec<QuiltLoaderVersion>> {
        Ok(http::fetch::fetch("https://meta.quiltmc.org/v3/versions/loader").await?)
    }

    pub async fn get_forge_metadata(&self) -> crate::Result<ForgeVersionList> {
        Ok(http::fetch::fetch::<ForgeVersionList>(
            "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json",
        )
        .await?)
    }

    pub async fn install_minecraft(
        &self,
        version: String,
        path: String,
        auth: AuthMethod,
    ) -> crate::Result<()> {
        let config = ConfigBuilder::new(PathBuf::from(path), version, auth).build();

        install(&config, Some(&EMITTER)).await?;

        Ok(())
    }
}
