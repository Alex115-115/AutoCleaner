use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
};

const CONFIG_FILE_NAME: &str = "tracked_folders.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct TrackedFolder {
    pub path: String,
    pub days: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FolderConfig {
    pub folders: Vec<TrackedFolder>,
}

impl FolderConfig {
    pub fn load() -> Self {
        let config_path = get_config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            FolderConfig::default()
        }
    }
}

pub fn get_app_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autocleaner")
}

pub fn get_config_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("autocleaner").join(CONFIG_FILE_NAME)
}

pub fn save_config(config: &FolderConfig) {
    let config_path = get_config_path();

    if let Some(parent) = config_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(mut file) = File::create(&config_path) {
        if let Err(e) = serde_json::to_writer_pretty(&mut file, config) {
            eprintln!("Failed to write config: {}", e);
        }
    }
}
