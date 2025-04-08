use crate::CREATE_NO_WINDOW;
use std::{os::windows::process::CommandExt, path::PathBuf};

/// Returns the path to the AutoCleaner startup shortcut on Windows.
///
/// # Returns
///
/// - A [`PathBuf`] pointing to the expected location of `AutoCleaner.lnk`.
///
#[cfg(target_os = "windows")]
pub fn get_startup_shortcut_path() -> PathBuf {
    let startup = std::env::var("APPDATA").unwrap_or_default(); // C:\Users\<user>\AppData\Roaming
    PathBuf::from(startup)
        .join("Microsoft\\Windows\\Start Menu\\Programs\\Startup\\AutoCleaner.lnk")
}

/// Enables or disables AutoCleaner running at Windows startup.
///
/// This function creates or removes a `.lnk` shortcut in the Windows Startup folder
/// to control whether the app launches automatically when the user logs into Windows.
///
/// - When `enabled` is `true`, a shortcut to `AutoCleaner.exe` is created with `--tray-startup` argument.
/// - When `enabled` is `false`, the shortcut is removed if it exists.
///
/// # Parameters
///
/// - `enabled`: If `true`, adds the app to startup. If `false`, removes it.
///
#[cfg(target_os = "windows")]
pub fn set_startup(enabled: bool) {
    use std::process::Command;

    let shortcut_path = get_startup_shortcut_path();

    if enabled {
        let exe_path = PathBuf::from("C:\\Program Files\\AutoCleaner\\autocleaner.exe");
        let target = exe_path.to_string_lossy();
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-WindowStyle",
                "Hidden",
                "-Command",
                &format!(
                    r#"$s=(New-Object -COM WScript.Shell).CreateShortcut('{}');$s.TargetPath='{}';$s.Arguments='--tray-startup';$s.Save()"#,
                    shortcut_path.display(),
                    target
                ),
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    } else {
        let _ = std::fs::remove_file(shortcut_path);
    }
}

/// Dummy implementation of [`set_startup`] for non-Windows platforms.
///
/// This function exists to provide cross-platform compatibility but does nothing
/// on platforms other than Windows.
///
/// # Parameters
///
/// - `_enabled`: Ignored. Present only to match the Windows function signature.
///
#[cfg(not(target_os = "windows"))]
pub fn set_startup(_enabled: bool) {}

/// Checks whether AutoCleaner is set to run at Windows startup.
///
/// This function checks if the startup shortcut file (`AutoCleaner.lnk`) exists
/// in the user's Windows Startup folder.
///
/// # Returns
///
/// - `true` if the startup shortcut exists.
/// - `false` otherwise.
///
pub fn is_startup_enabled() -> bool {
    get_startup_shortcut_path().exists()
}
