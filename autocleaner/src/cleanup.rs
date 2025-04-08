use chrono::{Duration, Utc};
use std::fs;
use walkdir::WalkDir;

/// Scans a folder and counts the number of files older than the given number of days.
///
/// This function walks recursively through the given directory path and counts
/// all regular files whose last modified timestamp is older than the specified threshold.
///
/// # Parameters
///
/// - `path`: The path to the folder that should be scanned.
/// - `days`: The age threshold in days. Files modified before this threshold are counted.
///
/// # Returns
///
/// The number of files older than `days` days found in the specified folder.
///
pub fn scan_folder(path: &str, days: u32) -> usize {
    let threshold = Utc::now() - Duration::days(days.into());
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            if let Ok(metadata) = fs::metadata(e.path()) {
                if let Ok(modified) = metadata.modified() {
                    let mod_time = chrono::DateTime::<Utc>::from(modified);
                    return mod_time < threshold;
                }
            }
            false
        })
        .count()
}

/// Removes files older than the given number of days from the specified folder.
///
/// This function recursively traverses the folder at the given path and deletes all
/// regular files whose last modified time is older than the specified threshold.
///
/// # Parameters
///
/// - `path`: The path to the folder where old files should be removed.
/// - `days`: The age threshold in days. Files older than this will be deleted.
///
/// # Returns
///
/// The number of files successfully removed.
///
pub fn remove_old_files(path: &str, days: u32) -> usize {
    let threshold = Utc::now() - Duration::days(days.into());
    let mut removed = 0;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(metadata) = fs::metadata(entry.path()) {
            if let Ok(modified) = metadata.modified() {
                let mod_time = chrono::DateTime::<Utc>::from(modified);
                if mod_time < threshold {
                    let _ = fs::remove_file(entry.path());
                    removed += 1;
                }
            }
        }
    }

    removed
}
