use std::{env, fs, path::Path, process::Command};
use winreg::{enums::*, RegKey};

/// Installer script for AutoCleaner (Windows).
///
/// # Steps Performed
///
/// 1. **Build** the `autocleaner` crate in release mode.
/// 2. **Copy** the resulting `.exe` to `Program Files\AutoCleaner`.
/// 3. **Determine install location** using the `ProgramFiles` environment variable.
/// 4. **Create install directory** if it doesn't already exist (`C:\Program Files\AutoCleaner`).
/// 5. **Copy** the `icon.ico` to the same directory.
/// 6. **Register** the application for toast notifications in the Windows Registry.
///
fn main() {
    println!("Running installer task...");

    // Build in release mode
    let status = Command::new("cargo")
        .args(["build", "--release", "--package", "autocleaner"])
        .status()
        .expect("Failed to build project");
    if !status.success() {
        panic!("Build failed.");
    }
    println!("Build successful.");

    // Get path to built binary
    let exe_path = Path::new("target/release/autocleaner.exe");
    if !exe_path.exists() {
        panic!("Built binary not found at {:?}", exe_path);
    }

    // Install location
    let program_files = env::var("ProgramFiles").expect("ProgramFiles env not found");
    let install_dir = Path::new(&program_files).join("AutoCleaner");

    // Create install dir
    if !install_dir.exists() {
        fs::create_dir_all(&install_dir)
            .expect("Failed to create install dir [USE autocleaner_setup.bat]");
    }

    // Copy binary
    let target_exe = install_dir.join("autocleaner.exe");
    fs::copy(exe_path, &target_exe).expect("Failed to copy binary [USE autocleaner_setup.bat]");

    // Copy icon
    let icon_path = install_dir.join("icon.ico");
    fs::copy("resources/icon.ico", &icon_path)
        .expect("Failed to copy icon.ico [USE autocleaner_setup.bat]");

    // Registry for toast notification
    setup_registry_for_notifications(&icon_path.to_string_lossy());

    println!("Installed to {:?}", install_dir);
}

/// Configures the Windows registry for toast notifications using an AppUserModelID.
///
/// This function writes registry entries under:
/// `HKEY_CLASSES_ROOT\AppUserModelId\AutoCleaner.App`
///
/// It registers the application's display name and icon, which are required
/// for Windows to properly associate toast notifications with the application.
///
/// # Parameters
///
/// - `icon_path`: The full path to the `.ico` file that should appear in the notification.
///
fn setup_registry_for_notifications(icon_path: &str) {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let (key, _) = hkcr
        .create_subkey("AppUserModelId\\AutoCleaner.App")
        .expect("Couldn't create registry key [USE autocleaner_setup.bat]");

    key.set_value("DisplayName", &"AutoCleaner").unwrap();
    key.set_value("IconUri", &icon_path).unwrap();

    println!("Registry configured for toast notifications.");
}
