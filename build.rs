const COMMANDS: &[&str] = &[
    "get_vanilla_versions",
    "get_fabric_versions",
    "get_fabric_loaders",
    "get_quilt_versions",
    "get_quilt_loaders",
    "get_forge_metadata",
    "install_minecraft",
    "launch_minecraft"
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
