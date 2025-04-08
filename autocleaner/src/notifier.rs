use crate::{
    cleanup::scan_folder, config::FolderConfig, config::TrackedFolder, env, settings::get_exec_path,
};
use std::path::Path;
use win_toast_notify::{Action, ActivationType, WinToastNotify};

/// Displays a Windows toast notification if expired files are found.
///
/// This function triggers a toast notification using `WinToastNotify` to inform the user
/// that expired files were found during a scan. If no expired files are found (`count == 0`),
/// the function exits no notification is shown.
///
/// The notification includes:
/// - A message indicating how many expired files were found.
/// - An action button that opens the folder containing the executable.
///
/// # Parameters
///
/// - `count`: The number of expired files found. If zero, no notification is shown.
/// - `exe_path`: The path to the current executable. Used to derive the folder to open.
///
pub fn notify_expired_files(count: usize, exe_path: String) {
    if count == 0 {
        return;
    }

    let folder_path = Path::new(&exe_path)
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_string_lossy()
        .to_string();

    let message = format!("🧹 {} expired files found", count);

    WinToastNotify::new()
        .set_app_id("Autocleaner.App")
        .set_title("AutoCleaner")
        .set_messages(vec![&message])
        .set_actions(vec![Action {
            activation_type: ActivationType::Protocol,
            action_content: "Open AutoCleaner".to_string(),
            arguments: folder_path,
            image_url: None,
        }])
        .show()
        .expect("Failed to show toast notification");
}

/// Scans all tracked folders for expired files and sends a notification if any are found.
///
/// This function:
/// - Loads the current [`FolderConfig`] from disk.
/// - Scans all tracked folders for files older than their configured threshold.
/// - If expired files are found, it triggers a toast notification via [`notify_expired_files`].
///
/// The path to the current executable is used to construct a folder path that is passed
/// to the notification action (so the app can be reopened easily from the toast).
///
pub fn scan_and_notify() {
    let path = get_exec_path()
        .unwrap_or_else(|| env::current_exe().unwrap().to_string_lossy().to_string());

    let folders = FolderConfig::load().folders;
    let count = count_expired_from_folders(&folders);

    notify_expired_files(count, path);
}

/// Counts all expired files across a list of tracked folders.
///
/// This function iterates over each [`TrackedFolder`] in the provided slice and
/// sums up the number of files that are older than the specified threshold (`days`)
/// in each folder, using [`scan_folder`].
///
/// # Parameters
///
/// - `folders`: A slice of [`TrackedFolder`] items representing the folders to scan.
///
/// # Returns
///
/// - The total number of expired files found across all folders.
///
pub fn count_expired_from_folders(folders: &[TrackedFolder]) -> usize {
    folders.iter().map(|f| scan_folder(&f.path, f.days)).sum()
}
