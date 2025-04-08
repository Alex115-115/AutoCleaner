use fs4::FileExt;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

/// A guard that ensures only a single instance of the application is running.
///
/// # Fields
///
/// - `_file`: A file handle used to maintain the lock.
///
pub struct SingletonGuard {
    _file: File,
}

/// Attempts to acquire a lock to ensure a single instance of the application is running.
///
/// # Parameters
///
/// - `lock_path`: The path to the lock file used for controlling single instance behavior.
///
/// # Returns
///
/// - `Some(SingletonGuard)` if the lock is acquired successfully.
/// - `None` if the file cannot be opened or the exclusive lock cannot be acquired
///
pub fn acquire_singleton_lock(lock_path: PathBuf) -> Option<SingletonGuard> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(lock_path)
        .ok()?;

    if file.try_lock_exclusive().is_ok() {
        Some(SingletonGuard { _file: file })
    } else {
        None
    }
}
