use std::io;
use std::path::PathBuf;

use directories::ProjectDirs;

/// Returns the data path, creating the directory if needed, using XDG or platform conventions.
pub fn get_script_path(file_name: &str) -> Result<PathBuf, io::Error> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "txtransform", "TXTransform") {
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
