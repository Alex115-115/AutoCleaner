use crate::{
    cleanup::{remove_old_files, scan_folder},
    config::{save_config, FolderConfig, TrackedFolder},
    fs, get_config_path,
    startup::{get_startup_shortcut_path, is_startup_enabled, set_startup},
};

/// The main application struct for AutoCleaner.
///
/// `AutoCleanerApp` holds the state and configuration for the application, including:
/// - The folder configuration used to determine which directories to monitor or clean.
/// - A log string that captures recent activity or output.
/// - A flag indicating whether the app should automatically run at startup.
///
/// # Fields
///
/// - `config`: The [`FolderConfig`] used to store information about which folders should be cleaned and how.
/// - `log`: A string buffer containing the latest log output, usually updated after operations.
/// - `run_at_startup`: Whether the app should be scheduled to run automatically on system startup.
///
pub struct AutoCleanerApp {
    pub config: FolderConfig,
    pub log: String,
    pub run_at_startup: bool,
}

impl Default for AutoCleanerApp {
    /// Creates a default instance of [`AutoCleanerApp`].
    ///
    /// This implementation attempts to load the saved folder configuration from disk.
    /// If the config file exists and can be read and parsed, it is used.
    /// Otherwise, a default [`FolderConfig`] is used instead.
    ///
    /// The `run_at_startup` flag is initialized based on the presence of a startup shortcut.
    /// The `log` field is initialized as an empty string.
    ///
    /// # Behavior
    /// - Reads the config from the path returned by [`get_config_path()`].
    /// - Parses the config using `serde_json`. If parsing fails, it falls back to `FolderConfig::default()`.
    /// - Checks for the existence of the startup shortcut via [`get_startup_shortcut_path()`].
    ///
    fn default() -> Self {
        let config_path = get_config_path();
        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            FolderConfig::default()
        };

        Self {
            run_at_startup: get_startup_shortcut_path().exists(),
            config,
            log: String::new(),
        }
    }
}

impl eframe::App for AutoCleanerApp {
    /// Updates the user interface on each frame.
    ///
    /// This method is called by `eframe` every frame to redraw the GUI and handle user interactions.
    /// It includes:
    /// - Handling window events.
    /// - Displaying and updating tracked folders.
    /// - Adding new folders to track using a folder picker.
    /// - Running scans and cleanup operations based on file age.
    /// - Managing Windows startup behavior.
    /// - Displaying a scrollable log of actions and events.
    ///
    /// # UI Breakdown
    ///
    /// - **Add Folder**: Opens a folder picker and adds the selected folder to tracking, if not already tracked.
    /// - **Tracked Folders**: Shows a list of currently tracked folders with:
    ///   - A slider to set how many days old files must be to qualify for scanning/deletion.
    ///   - Buttons to scan, delete, or untrack each folder.
    /// - **Startup Toggle**: Lets the user choose whether the app should run at Windows startup.
    /// - **Log Viewer**: A scrollable area where recent events (like added folders or file deletions) are displayed.
    ///
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        use eframe::egui::{CentralPanel, ScrollArea};

        for event in ctx.input(|i| i.viewport().events.clone()) {
            if matches!(event, eframe::egui::ViewportEvent::Close) {}
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("AutoCleaner");
            if ui.button("➕ Add Folder to Track").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    let path_str = path.to_string_lossy().to_string();
                    if !self.config.folders.iter().any(|f| f.path == path_str) {
                        self.config.folders.push(TrackedFolder {
                            path: path_str.clone(),
                            days: 200,
                        });
                        save_config(&self.config);
                        self.log.push_str(&format!("✔ Added: {}\n", path_str));
                    } else {
                        self.log
                            .push_str(&format!("⚠ Already tracked: {}\n", path_str));
                    }
                }
            }

            ui.separator();
            ui.label("📁 Tracked folders:");

            let mut to_remove = None;
            for (index, folder) in self.config.folders.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label("📂");
                    ui.label(&folder.path);

                    ui.label("Days:");
                    ui.add(eframe::egui::Slider::new(&mut folder.days, 1..=365));

                    if ui.button("🔍 Scan").clicked() {
                        let count = scan_folder(&folder.path, folder.days);
                        self.log.push_str(&format!(
                            "🔍 {} files older than {} days in {}\n",
                            count, folder.days, folder.path
                        ));
                    }

                    if ui.button("🗑 Remove").clicked() {
                        let deleted = remove_old_files(&folder.path, folder.days);
                        self.log.push_str(&format!(
                            "🗑 Removed {} files older than {} days from {}\n",
                            deleted, folder.days, folder.path
                        ));
                    }

                    if ui.button("❌ Untrack").clicked() {
                        to_remove = Some(index);
                    }
                });
            }

            if let Some(index) = to_remove {
                let removed = self.config.folders.remove(index);
                save_config(&self.config);
                self.log.push_str(&format!(
                    "❌ Folder removed from tracking: {}\n",
                    removed.path
                ));
            }

            save_config(&self.config);

            ui.separator();
            self.run_at_startup = is_startup_enabled();

            if ui
                .checkbox(&mut self.run_at_startup, "Run at Windows startup")
                .clicked()
            {
                set_startup(self.run_at_startup);
                self.log.push_str(&format!(
                    "🔁 Startup {}abled\n",
                    if self.run_at_startup { "en" } else { "dis" }
                ));
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("📝 Log:");
                if ui.button("🗑 Clear Log").clicked() {
                    self.log.clear();
                }
            });

            ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                ui.monospace(&self.log);
            });
        });
    }
}
