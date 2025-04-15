//! Submodule for test utilities.

use std::path::Path;

/// Copy a directory recursively.
///
/// # Arguments
///
/// * `src` - The source directory to copy.
/// * `dst` - The destination directory where the source will be copied.
///
/// # Errors
///
/// * When the source directory does not exist.
/// * When the destination directory cannot be created.
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
