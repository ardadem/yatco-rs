use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

pub fn transform(
    input: &str,
    _extra_args: Option<HashMap<String, String>>,
) -> Result<String, Box<dyn Error>> {
    // First parse the json from the string
    let parsed: serde_json::Value = serde_json::from_str(input)?;

    // Then serialize it pretty printed
    let mut buf: Vec<u8> = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    parsed.serialize(&mut ser)?;

    // And return
    Ok(String::from_utf8(buf)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_pretty_json() {
        let input = "{\"a\":1,\"b\":[2,3]}";
        let expected = "{\n    \"a\": 1,\n    \"b\": [\n        2,\n        3\n    ]\n}";
        assert_eq!(transform(input, None).unwrap(), expected);
    }

    #[test]
    fn test_transform_empty_object() {
        let input = "{}";
        let expected = "{}";
        assert_eq!(transform(input, None).unwrap(), expected);
    }

    #[test]
    fn test_transform_empty_array() {
        let input = "[]";
        let expected = "[]";
        assert_eq!(transform(input, None).unwrap(), expected);
    }
}
