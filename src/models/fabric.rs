use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FabricVersionList {
    pub game: Vec<FabricGameVersion>,
    pub loader: Vec<FabricLoaderVersion>,
}

#[derive(Serialize, Deserialize)]
pub struct FabricGameVersion {
    version: String,
    stable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct FabricLoaderVersion {
    seperator: String,
    build: u16,
    maven: String,
    version: String,
    stable: bool,
}
