use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QuiltVersionList {
    pub game: Vec<QuiltGameVersion>,
    pub loader: Vec<QuiltLoaderVersion>,
}

#[derive(Serialize, Deserialize)]
pub struct QuiltGameVersion {
    version: String,
    stable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct QuiltLoaderVersion {
    separator: String,
    build: u16,
    maven: String,
    version: String,
}
