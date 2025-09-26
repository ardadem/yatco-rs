use std::io;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

/// Returns the script path. If `file_name` is an absolute path and exists, return it directly.
/// Otherwise fall back to the app data directory and join the file_name there.
pub fn get_script_path(file_name: &str) -> Result<PathBuf, io::Error> {
    let candidate = Path::new(file_name);
    if candidate.is_absolute() {
        if candidate.exists() {
            return Ok(candidate.to_path_buf());
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Script path '{}' does not exist", file_name),
            ));
        }
    }

    if let Some(proj_dirs) = ProjectDirs::from("com", "yatco-rs", "yatco-rs") {
        let data_dir = proj_dirs.data_dir();
        let _ = std::fs::create_dir_all(data_dir); // Ensure directory exists
        let data_path = data_dir.join(file_name);
        Ok(data_path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine data directory",
        ))
    }
}
