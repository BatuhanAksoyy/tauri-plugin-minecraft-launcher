const COMMANDS: &[&str] = &[
    "list_vanilla_versions",
    "list_fabric_versions",
    "list_fabric_loaders",
    "list_quilt_versions",
    "list_quilt_loaders",
    "list_forge_versions",
    "list_forge_loaders",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
