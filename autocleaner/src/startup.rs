use crate::CREATE_NO_WINDOW;
use std::{os::windows::process::CommandExt, path::PathBuf};

#[cfg(target_os = "windows")]
pub fn get_startup_shortcut_path() -> PathBuf {
    let startup = std::env::var("APPDATA").unwrap_or_default(); // C:\Users\<user>\AppData\Roaming
    PathBuf::from(startup)
        .join("Microsoft\\Windows\\Start Menu\\Programs\\Startup\\AutoCleaner.lnk")
}

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

#[cfg(not(target_os = "windows"))]
pub fn set_startup(_enabled: bool) {}

pub fn is_startup_enabled() -> bool {
    get_startup_shortcut_path().exists()
}
