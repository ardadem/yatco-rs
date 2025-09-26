use std::collections::HashMap;
use std::error::Error;

pub fn transform(
    input: &str,
    _extra_args: Option<HashMap<String, String>>,
) -> Result<String, Box<dyn Error>> {
    let s = input.trim();

    // If the whole input is a valid JSON value, handle string values specially
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(s) {
        match val {
            serde_json::Value::String(inner) => {
                // Unescape the inner string value and return it
                return Ok(unescape_json_string(&inner)?);
            }
            _ => {
                // For other valid JSON values (objects, arrays, numbers, bool, null)
                // return the original input unchanged
                return Ok(s.to_string());
            }
        }
    }

    // If wrapped in double quotes, strip them for unescaping
    let inner = if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    };

    match unescape_json_string(inner) {
        Ok(u) => Ok(u),
        Err(_) => {
            // Fallback: simple replace of escaped quotes and backslashes
            let fallback = inner.replace("\\\"", "\"").replace("\\\\", "\\");
            Ok(fallback)
        }
    }
}

fn unescape_json_string(s: &str) -> Result<String, Box<dyn Error>> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }

        // c == '\\' -> parse escape
        let next = chars.next().ok_or("Invalid escape sequence")?;
        match next {
            '"' => out.push('"'),
            '\\' => out.push('\\'),
            '/' => out.push('/'),
            'b' => out.push('\x08'),
            'f' => out.push('\x0C'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            'u' => {
                // parse 4 hex digits
                let mut codepoint: u32 = 0;
                for _ in 0..4 {
                    let h = chars.next().ok_or("Invalid \\u escape (too short)")?;
                    codepoint =
                        codepoint << 4 | h.to_digit(16).ok_or("Invalid hex digit in \\u escape")?;
                }

                if let Some(ch) = std::char::from_u32(codepoint) {
                    out.push(ch);
                } else {
                    return Err("Invalid unicode codepoint".into());
                }
            }
            other => {
                // Unknown escape, push the char as-is (keep the backslash semantics loose)
                out.push(other);
            }
        }
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_basic() {
        let input = "{\\\"test123\\\":\\\"/test/path\\\"}"; // {\"test123\":\"/test/path\"}
        let out = transform(input, None).unwrap();
        assert_eq!(out, "{\"test123\":\"/test/path\"}");
    }

    #[test]
    fn test_unescape_quotes_wrapped() {
        let input = "\"{\\\"a\\\":1}\""; // quoted string
        let out = transform(input, None).unwrap();
        assert_eq!(out, "{\"a\":1}");
    }

    #[test]
    fn test_unescape_unicode() {
        let input = "Hello \\u0041"; // Hello \u0041 -> Hello A
        let out = transform(input, None).unwrap();
        assert_eq!(out, "Hello A");
    }
}
