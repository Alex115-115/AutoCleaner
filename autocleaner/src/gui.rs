use crate::app::AutoCleanerApp;
use eframe::egui::{ViewportBuilder};

/// Launches the AutoCleaner GUI application.
pub fn launch_gui() {
    let mut viewport = ViewportBuilder::default().with_inner_size([1000.0, 600.0]);

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "AutoCleaner GUI",
        native_options,
        Box::new(|_cc| Box::new(AutoCleanerApp::default())),
    )
    .expect("Failed to launch GUI");
}
