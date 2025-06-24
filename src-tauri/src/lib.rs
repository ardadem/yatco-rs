use std::collections::HashMap;

use crate::config::TomlConfig;

pub mod config;
mod transformers;

#[tauri::command]
fn transform(input: &str, preset_name: &str) -> String {
    let mut text: String = input.to_string();

    let preset = get_preset_by_name(preset_name);
    for transformer in preset.1 {
        match transformers::get_transformer(&transformer) {
            Some(transformer_fn) => match transformer_fn(&text, preset.2.clone()) {
                Ok(result) => text = result,
                Err(e) => return format!("Error in transformer: {}", e),
            },
            None => return format!("Error! Transformer {} doesn't exist.", transformer),
        }
    }

    text
}

fn get_preset_by_name(name: &str) -> (String, Vec<String>, Option<HashMap<String, String>>) {
    let presets = config::PRESETS_MEM.read().unwrap();
    match presets.presets.iter().find(|p| p.name == name) {
        Some(preset) => (
            preset.name.clone(),
            preset.transformers.clone(),
            preset.extra_args.clone(),
        ),
        None => (String::new(), Vec::new(), None),
    }
}

#[tauri::command]
fn get_presets() -> Vec<(String, Vec<String>, Option<HashMap<String, String>>)> {
    let presets = config::PRESETS_MEM.read().unwrap();
    presets
        .presets
        .iter()
        .map(|p| (p.name.clone(), p.transformers.clone(), p.extra_args.clone()))
        .collect()
}

#[tauri::command]
fn add_preset(
    name: &str,
    transformer_list: Vec<String>,
    extra_args: Option<HashMap<String, String>>,
) {
    println!(
        "Adding preset, name: {}, transformer_list: {}, extra_args: {}",
        name,
        transformer_list.join(", "),
        extra_args
            .as_ref()
            .map_or("None".to_string(), |args| format!("{:?}", args))
    );
    let mut presets_mem = config::PRESETS_MEM.write().unwrap();
    presets_mem.presets.push(config::Preset {
        name: name.to_string(),
        transformers: transformer_list,
        extra_args: extra_args.clone(),
    });
    let _ = config::Presets::save(&presets_mem);
}

#[tauri::command]
fn delete_preset(name: &str) {
    println!("Deleting preset, name: {}", name);
    let mut presets_mem = config::PRESETS_MEM.write().unwrap();
    presets_mem.presets.retain(|p| p.name != name);
    let _ = config::Presets::save(&presets_mem);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load configurations
    {
        let mut config_mem = config::CONFIG_MEM.write().unwrap();
        *config_mem = config::Config::load().unwrap();
        println!("Config loaded: {:?}", config_mem);
    }
    {
        let mut presets_mem = config::PRESETS_MEM.write().unwrap();
        *presets_mem = config::Presets::load().unwrap();
        println!("Presets loaded: {:?}", presets_mem.presets);
    }

    // Then build app
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            transform,
            get_presets,
            add_preset,
            delete_preset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
