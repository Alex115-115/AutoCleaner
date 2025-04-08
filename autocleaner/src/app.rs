use crate::{
    cleanup::{remove_old_files, scan_folder},
    config::{save_config, FolderConfig, TrackedFolder},
    fs, get_config_path,
    startup::{get_startup_shortcut_path, is_startup_enabled, set_startup},
};

pub struct AutoCleanerApp {
    pub config: FolderConfig,
    pub log: String,
    pub run_at_startup: bool,
}

impl Default for AutoCleanerApp {
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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        use eframe::egui::{CentralPanel, ScrollArea};

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
