use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use toml;

pub trait TomlConfig: Sized + Serialize + for<'de> Deserialize<'de> + Default {
    const FILE_NAME: &'static str;

    fn load() -> io::Result<Self> {
        match get_config_path(Self::FILE_NAME) {
            Ok(path) => {
                if path.exists() {
                    let contents = fs::read_to_string(path.to_str().unwrap_or_default())?;
                    let config: Self = toml::from_str(&contents)
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                    return Ok(config);
                }
            }
            Err(_) => {}
        }

        // Load defaults
        Ok(Self::default())
    }

    fn save(&self) -> io::Result<()> {
        match get_config_path(Self::FILE_NAME) {
            Ok(path) => {
                let toml_str = toml::to_string_pretty(self)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                let mut file = fs::File::create(path)?;
                file.write_all(toml_str.as_bytes())?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub theme: String,
    pub font_size: u8,
}

impl TomlConfig for Config {
    const FILE_NAME: &'static str = "config.toml";
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Preset {
    pub name: String,
    pub transformers: Vec<String>,
    pub extra_args: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Presets {
    pub presets: Vec<Preset>,
}

impl TomlConfig for Presets {
    const FILE_NAME: &'static str = "presets.toml";
}

/// Returns the config file path, creating the directory if needed, using XDG or platform conventions.
fn get_config_path(config_name: &str) -> Result<PathBuf, io::Error> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "yatco-rs", "yatco-rs") {
        let config_dir = proj_dirs.config_dir();
        let _ = std::fs::create_dir_all(config_dir); // Ensure directory exists
        let config_path = config_dir.join(config_name);
        Ok(config_path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine config directory",
        ))
    }
}
