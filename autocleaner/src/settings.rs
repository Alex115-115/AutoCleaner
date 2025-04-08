use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

/// Represents general settings for the application.
///
/// # Fields
///
/// - `exec_path`: Optional path to the app's executable (`.exe`) as a `String`.
///
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub exec_path: Option<String>,
}

/// Returns the path to the application's settings file.
///
/// # Returns
///
/// - A [`PathBuf`] pointing to the `settings.json` file.
///
pub fn get_settings_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("autocleaner").join("settings.json")
}

/// Loads application settings from the settings file.
///
/// # Returns
///
/// - A [`Settings`] instance, either loaded from disk or initialized with default values.
///
pub fn load_settings() -> Settings {
    let path = get_settings_path();
    if let Ok(content) = fs::read_to_string(path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        Settings::default()
    }
}

/// Saves the executable path to the application settings file.
///
/// # Parameters
///
/// - `path`: A `String` representing the full path to the application executable.
///
/// # Returns
///
/// - `io::Result<()>` — returns `Ok(())` if the settings were saved successfully,
///   or an `Err` if any I/O error occurred during writing.
///
pub fn save_exec_path(path: String) -> io::Result<()> {
    let mut settings = load_settings();
    settings.exec_path = Some(path);

    let settings_path = get_settings_path();
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let json = serde_json::to_string_pretty(&settings)?;
    fs::write(settings_path, json)?;
    Ok(())
}

/// Retrieves the saved executable path from the application settings.
///
/// # Returns
///
/// - `Some(String)` containing the executable path, if set.
/// - `None` if no path has been saved.
///
pub fn get_exec_path() -> Option<String> {
    load_settings().exec_path
}
