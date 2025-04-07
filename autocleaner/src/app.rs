
pub struct AutoCleanerApp {
    pub log: String,
}

impl Default for AutoCleanerApp {
    fn default() -> Self {
        Self {
            log: String::new(),
        }
    }
}

impl eframe::App for AutoCleanerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        use eframe::egui::{CentralPanel, ScrollArea};

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("AutoCleaner");
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
