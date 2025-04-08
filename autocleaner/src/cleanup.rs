use chrono::{Duration, Utc};
use std::fs;
use walkdir::WalkDir;

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
