use crate::config::{Config, Presets};
use once_cell::sync::Lazy;
use std::sync::RwLock;

pub static CONFIG_MEM: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));
pub static PRESETS_MEM: Lazy<RwLock<Presets>> = Lazy::new(|| RwLock::new(Presets::default()));
