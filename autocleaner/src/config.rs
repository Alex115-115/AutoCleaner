use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
};

/// The file name used to store the tracked folders configuration.
///
/// This constant defines the name of the JSON file where the app saves and loads
/// folder tracking data
///
const CONFIG_FILE_NAME: &str = "tracked_folders.json";

/// Represents a folder that is being tracked by the application.
///
/// A `TrackedFolder` contains the path to a folder and a threshold in days,
/// indicating how old a file must be to be considered for scanning or deletion.
///
/// # Fields
///
/// - `path`: The full filesystem path to the folder.
/// - `days`: The number of days to use as the threshold for old files.
///
/// # See Also
///
/// - [`FolderConfig`] — holds a collection of `TrackedFolder` items.
///
#[derive(Serialize, Deserialize, Clone)]
pub struct TrackedFolder {
    pub path: String,
    pub days: u32,
}

/// Holds the configuration for all tracked folders in the application.
///
/// `FolderConfig` wraps a collection of [`TrackedFolder`] items, representing
/// all directories that the app is currently monitoring for cleanup purposes.
///
/// # Fields
///
/// - `folders`: A list of folders to track, each with its own path and age threshold.
///
/// - [`TrackedFolder`] — individual folder entry with path and cleanup threshold.
///
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FolderConfig {
    pub folders: Vec<TrackedFolder>,
}

impl FolderConfig {
    /// Loads the folder configuration from disk.
    ///
    /// This method attempts to read and deserialize the contents of the config file
    ///
    /// If the config file doesn't exist or cannot be read/parsed, it returns
    /// a default configuration instead.
    ///
    /// # Returns
    ///
    /// A valid `FolderConfig`, either loaded from disk or created with default values.
    ///
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

/// Retrieves user's configuration directory.
///
/// The returned path is used for storing application state, configuration files, and other
/// data that should be persisted across application runs.
///
/// # Returns
///
/// - A [`PathBuf`] representing the path to the application's data directory.
///
pub fn get_app_data_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("autocleaner")
}

/// Returns the path to the application's configuration file.
///
/// # Returns
///
/// - A [`PathBuf`] representing the full path to the config file.
///
pub fn get_config_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("autocleaner").join(CONFIG_FILE_NAME)
}

/// Saves the provided folder configuration to the configuration file.
///
/// # Parameters
///
/// - `config`: A reference to the [`FolderConfig`] to be saved.
///
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
