# AutoCleaner

AutoCleaner is a lightweight Windows desktop application designed to help users automatically clean up old files from user-defined folders. It provides a graphical user interface, system tray integration, toast notifications, and persistent configuration — all implemented in Rust.

---

## Features

- Track folders and define custom expiration thresholds (in days)
- One-click scan and file removal
- Configurable startup option (via system tray or GUI)
- Toast notifications when expired files are found
- System tray integration with dynamic context menu
- Prevents multiple GUI or tray instances (singleton protection)
- Self-contained installer with registry integration
- Native, modern GUI built with `eframe` and `egui`

---

## Installation

To install AutoCleaner on your system, run the following batch script with administrator rights:

```bat
autocleaner_setup.bat
```
## Installer Script

This script will:

- Build the application in release mode  
- Copy the binary and icon to `C:\Program Files\AutoCleaner`  
- Save the current executable path to settings  
- Set up Windows Registry entries for toast notifications  

> Run this with administrator rights to write to Program Files and modify the registry.

---

## Usage

The executable supports multiple modes:

| Command                         | Description                                  |
|--------------------------------|----------------------------------------------|
| `autocleaner.exe --gui`        | Launches the main graphical user interface   |
| `autocleaner.exe --tray`       | Starts the tray icon only                    |
| `autocleaner.exe --tray-startup` | Performs a scan + notification, then tray   |
| `autocleaner.exe`              | Launches both GUI and tray                   |

These modes are handled inside `main()` by parsing `env::args()`  
and protected by lock files (`gui.lock`, `tray.lock`) to prevent duplicate instances.

---

## Configuration

AutoCleaner stores its data in the system config directory:

- **Folder tracking:**  
  `%APPDATA%\AutoCleaner\tracked_folders.json`

- **General settings:**  
  `%APPDATA%\AutoCleaner\settings.json`

- **Startup shortcut:**  
  `%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\AutoCleaner.lnk`

- **Registry for toast notifications:**  
  `HKEY_CLASSES_ROOT\AppUserModelId\AutoCleaner.App`

---

## Technical Details

This section highlights the architectural and implementation aspects of the project.

### Technologies Used

- Rust — core language
- `eframe` / `egui` — GUI framework
- `tray-icon` — native system tray integration
- `shared_memory` — used for singleton flags across processes (`AtomicBool` in shared memory)
- `fs2` — file-based locking to ensure singleton behavior
- `winreg` — registry manipulation for toast notifications
- `walkdir` — recursive file traversal
- `chrono` — datetime handling for expiration logic
- `serde` / `serde_json` — serialization of config and settings
- `rfd` — native folder picker dialog

### Architecture Overview

- **AutoCleanerApp**: Main application state, implements `eframe::App`
- **TrackedFolder**: Model representing a single folder + expiration setting
- **FolderConfig**: Wrapper around a vector of tracked folders (saved as JSON)
- **Settings**: Stores general settings like the current executable path
- **GUI Mode**: Graphical interface built with `eframe` / `egui`, allows folder management, scanning, cleaning, and startup toggle
- **Tray Mode**: System tray with dynamic menu built using `tray-icon`, allows access to GUI, startup toggle, and exit
- **Startup Management**: Toggled from both the GUI and system tray, which creates or removes a shortcut in the Windows Startup folder
- **Installer**: A custom `xtask` builds the app, copies the binary and icon, saves settings, and configures the Windows Registry for toast notifications
- **Notifications**: Triggered using `WinToastNotify` with a custom `AppUserModelID`

### Process Isolation

- Tray and GUI modes use separate singleton locks (`gui.lock`, `tray.lock`)
- The `shared_memory` crate is integrated but not actively used yet. It's included as a foundation for potential future inter-process coordination between GUI and tray components.


---

## License

This project is open-sourced under the MIT License. Use at your own risk.  
You are free to use, modify, and distribute it.  
The author assumes no responsibility for any damage or data loss resulting from its use.

---
