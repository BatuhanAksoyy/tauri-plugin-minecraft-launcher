use std::{path::PathBuf, sync::Arc};

use fabric::{FabricGameVersion, FabricLoaderVersion};
use forge::ForgeVersionList;
use lyceris::{
    auth::AuthMethod,
    http,
    minecraft::{
        config::{Memory, Profile},
        emitter::{Emitter, Event},
        install::install,
        loader::Loader,
    },
};
use quilt::{QuiltGameVersion, QuiltLoaderVersion};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::Emitter as TauriEmitter;
use tauri::{async_runtime::Mutex, plugin::PluginApi, AppHandle, Runtime};
use tokio::process::Child;
use vanilla::{VanillaGameVersion, VanillaVersionList};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    child: Arc<Mutex<Option<Child>>>,
    emitter: Emitter,
    _api: PluginApi<R, C>,
) -> crate::Result<MinecraftLauncher<R>> {
    Ok(MinecraftLauncher {
        app: app.clone(),
        child,
        emitter,
    })
}

/// Access to the minecraft-launcher APIs.
pub struct MinecraftLauncher<R: Runtime> {
    app: AppHandle<R>,
    child: Arc<Mutex<Option<Child>>>,
    emitter: Emitter,
}

impl<R: Runtime> MinecraftLauncher<R> {
    pub async fn get_vanilla_versions(&self) -> crate::Result<Vec<VanillaGameVersion>> {
        Ok(http::fetch::fetch::<VanillaVersionList>(
            "https://launchermeta.mojang.com/mc/game/version_manifest.json",
            None,
        )
        .await?
        .versions)
    }

    pub async fn get_fabric_versions(&self) -> crate::Result<Vec<FabricGameVersion>> {
        Ok(http::fetch::fetch("https://meta.fabricmc.net/v2/versions/game", None).await?)
    }

    pub async fn get_fabric_loaders(&self) -> crate::Result<Vec<FabricLoaderVersion>> {
        Ok(http::fetch::fetch("https://meta.fabricmc.net/v2/versions/loader", None).await?)
    }

    pub async fn get_quilt_versions(&self) -> crate::Result<Vec<QuiltGameVersion>> {
        Ok(http::fetch::fetch("https://meta.quiltmc.org/v3/versions/game", None).await?)
    }

    pub async fn get_quilt_loaders(&self) -> crate::Result<Vec<QuiltLoaderVersion>> {
        Ok(http::fetch::fetch("https://meta.quiltmc.org/v3/versions/loader", None).await?)
    }

    pub async fn get_forge_metadata(&self) -> crate::Result<ForgeVersionList> {
        Ok(http::fetch::fetch::<ForgeVersionList>(
            "https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json",
            None,
        )
        .await?)
    }

    pub async fn install_minecraft(&self, config: Config) -> crate::Result<()> {
        install(&config.into(), Some(&self.emitter)).await?;
        Ok(())
    }

    pub async fn launch_minecraft(&self, config: Config) -> crate::Result<()> {
        let child = lyceris::launch(&config.into(), Some(&self.emitter)).await?;
        self.child.lock().await.replace(child);

        self.emitter
            .on(Event::SingleDownloadProgress, {
                let app = self.app.clone();
                move |(path, current, total): (String, u64, u64)| {
                    app.emit("minecraft-single-progress", (path, current, total))
                        .expect("failed to emit minecraft-console event");
                }
            })
            .await;

        self.emitter
            .on(Event::MultipleDownloadProgress, {
                let app = self.app.clone();
                move |(current, total): (u64, u64)| {
                    app.emit("minecraft-multi-progress", (current, total))
                        .expect("failed to emit minecraft-console event");
                }
            })
            .await;

        self.emitter
            .on(Event::Console, {
                let app = self.app.clone();
                move |line: String| {
                    app.emit("minecraft-console", line)
                        .expect("failed to emit minecraft-console event");
                }
            })
            .await;

        tauri::async_runtime::spawn({
            let app = self.app.clone();
            let child = self.child.clone();
            async move {
                loop {
                    if let Some(child) = child.lock().await.as_mut() {
                        if let Ok(Some(status)) = child.try_wait() {
                            if status.success() {
                                app.emit("minecraft-exit", true)
                                    .map_err(crate::Error::from)?;
                            } else {
                                app.emit("minecraft-exit", false)
                                    .map_err(crate::Error::from)?;
                            }
                            break;
                        }
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
                Ok::<(), crate::Error>(())
            }
        });

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub game_dir: PathBuf,
    pub version: String,
    pub authentication: AuthMethod,
    pub memory: Option<Memory>,
    pub version_name: Option<String>,
    pub profile: Option<Profile>,
    pub loader: Option<LoaderConfig>,
    pub java_version: Option<String>,
    pub runtime_dir: Option<PathBuf>,
    pub custom_java_args: Vec<String>,
    pub custom_args: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LoaderConfig {
    pub r#type: String,
    pub version: String,
}

impl From<Config> for lyceris::Config<Box<dyn Loader>> {
    fn from(config: Config) -> Self {
        lyceris::Config {
            game_dir: config.game_dir,
            version: config.version,
            authentication: config.authentication,
            memory: config.memory,
            version_name: config.version_name,
            profile: config.profile,
            loader: {
                if let Some(loader) = config.loader {
                    Some(match loader.r#type.to_lowercase().as_str() {
                        "fabric" => {
                            Box::new(lyceris::minecraft::loader::fabric::Fabric(loader.version))
                        }
                        "quilt" => {
                            Box::new(lyceris::minecraft::loader::quilt::Quilt(loader.version))
                        }
                        "forge" => {
                            Box::new(lyceris::minecraft::loader::forge::Forge(loader.version))
                        }
                        "neoforge" => Box::new(lyceris::minecraft::loader::neoforge::NeoForge(
                            loader.version,
                        )),
                        _ => panic!("Invalid loader type: {}", loader.r#type),
                    })
                } else {
                    None
                }
            },
            java_version: config.java_version,
            runtime_dir: config.runtime_dir,
            custom_java_args: config.custom_java_args,
            custom_args: config.custom_args,
            client: None,
        }
    }
}
