use crate::app::AutoCleanerApp;
use eframe::egui::{IconData, ViewportBuilder};
use ico::IconDir;
use std::sync::Arc;

/// Loads an icon from a byte slice containing ICO data.
///
/// This function takes raw ICO bytes, decodes the first icon in the file, and
/// returns its RGBA data along with its width and height wrapped in an `Arc<IconData>`.
/// If the ICO data is invalid or the icon cannot be decoded, it returns `None`.
///
/// # Parameters
///
/// - `bytes`: A slice of bytes containing the ICO data to be decoded.
///
/// # Returns
///
/// - An `Option<Arc<IconData>>` containing the icon's RGBA data, width, and height if successful.
///   If any error occurs during the decoding process, `None` is returned.
///
fn load_icon_from_ico_bytes(bytes: &[u8]) -> Option<Arc<IconData>> {
    let cursor = std::io::Cursor::new(bytes);
    let icon_dir = IconDir::read(cursor).ok()?;
    let entry = icon_dir.entries().first()?;
    let image = entry.decode().ok()?;

    let width = image.width();
    let height = image.height();
    let rgba = image.rgba_data().to_vec();

    Some(Arc::new(IconData {
        rgba,
        width,
        height,
    }))
}

/// Launches the AutoCleaner GUI application.
pub fn launch_gui() {
    let mut viewport = ViewportBuilder::default().with_inner_size([1000.0, 600.0]);

    let icon_bytes = include_bytes!("../../resources/icon.ico");
    if let Some(icon) = load_icon_from_ico_bytes(icon_bytes) {
        viewport = viewport.with_icon(icon);
    }

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
