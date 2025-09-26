use std::collections::HashMap;
use std::error::Error;

mod custom_py;
mod json_unescape;
mod pretty_json;
mod util;

pub type TransformerFn =
    fn(&str, Option<HashMap<String, String>>) -> Result<String, Box<dyn Error>>;

// Import the transform functions
use custom_py::transform as custom_py_transform;
use json_unescape::transform as json_unescape_transform;
use pretty_json::transform as pretty_json_transform;

// Transformer names
const PRETTY_JSON_NAME: &str = "pretty_json";
const CUSTOM_PY_NAME: &str = "custom_py";
const JSON_UNESCAPE_NAME: &str = "json_unescape";

// Getter function which implements transformer name and function mapping
pub fn get_transformer(name: &str) -> Option<TransformerFn> {
    match name {
        PRETTY_JSON_NAME => Some(pretty_json_transform),
        CUSTOM_PY_NAME => Some(custom_py_transform),
        JSON_UNESCAPE_NAME => Some(json_unescape_transform),
        _ => None,
    }
}
