#![doc = include_str!("../README.md")]

use std::{hash::Hash, path::Path};
use std::hash::DefaultHasher;
use std::hash::Hasher;

/// Returns the last modified time of a file in seconds since the UNIX epoch.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the file.
///
/// # Returns
///
/// * `Some(u64)` - The last modified time in seconds since the UNIX epoch.
/// * `None` - If the file does not exist, is not a file, or if the modified
///   time could not be retrieved.
pub fn file_last_modified_time(file_path: &Path) -> Option<u64> {
    let Ok(metadata) = file_path.metadata() else {
        return None;
    };
    if !metadata.is_file() {
        return None;
    }
    let Ok(modified) = metadata.modified() else {
        return None;
    };
    let Ok(modified) = modified.duration_since(std::time::UNIX_EPOCH) else {
        return None;
    };
    Some(modified.as_secs())
}

/// Returns the most recent file in a directory with a specific extension.
/// If the directory contains subdirectories, it will recursively check them.
///
/// # Arguments
///
/// * `directory` - A reference to the path of the directory.
/// * `extensions` - A slice of string slices representing the file extensions
///   to check.
///
/// # Returns
///
/// * `Some(u64)` - The last modified time of the most recent file in seconds
///   since the UNIX epoch.
/// * `None` - If the directory does not exist, is not a directory, or if no
///   files with the specified extensions are found.
/// * If the directory is empty or contains no files with the specified
///   extensions, it will return `None`.
pub fn most_recent_file(directory: &Path, extensions: &[&str]) -> Option<u64> {
    let mut most_recent: Option<u64> = None;
    let Ok(directory) = std::fs::read_dir(directory) else {
        return None;
    };
    for entry in directory {
        let Ok(entry) = entry else {
            continue;
        };

        if entry.path().is_dir() {
            // If the entry is a directory, we recursively check it
            if let Some(most_recent_in_subdir) = most_recent_file(&entry.path(), extensions) {
                most_recent = most_recent
                    .map(|x| x.max(most_recent_in_subdir))
                    .or(Some(most_recent_in_subdir));
            }
            continue;
        }

        let Ok(file_name) = entry.file_name().into_string() else {
            continue;
        };

        // Check if the file has one of the specified extensions
        if !extensions.iter().any(|ext| file_name.ends_with(ext)) {
            continue;
        }

        let Some(modified) = file_last_modified_time(&entry.path()) else {
            continue;
        };

        most_recent = most_recent.map(|x| x.max(modified)).or(Some(modified));
    }

    most_recent
}

/// Returns the path to a cached file based on the directory and macro name.
///
/// # Arguments
///
/// * `salt` - A hash value used to generate a unique file name.
///
/// # Returns
///
/// * `PathBuf` - The path to the cached file.
pub fn cached_file_path<H: Hash>(salt: &H) -> std::path::PathBuf {
    // We compute the starting name of the file,
    // which is the Blake3 hash including the directory
    // and the name of the macro.
    let mut hasher = DefaultHasher::new();
    salt.hash(&mut hasher);
    env!("CARGO_PKG_VERSION").hash(&mut hasher);
    let hash = hasher.finish();
    let hash_hex = hash.to_string();

    // We check whether we already have a cached file in the temporary directory
    let tmp_dir = std::env::temp_dir();
    tmp_dir.join(format!("{hash_hex}.sql"))
}
