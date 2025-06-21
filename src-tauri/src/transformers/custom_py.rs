use super::util;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn transform(
    input: &str,
    extra_args: Option<HashMap<String, String>>,
) -> Result<String, Box<dyn Error>> {
    let py_script = extra_args.as_ref().and_then(|args| args.get("py_script"));
    if py_script.is_none() {
        return Err("Required extra arguments are not given to the transformer".into());
    }

    // Get the path to the Python script
    let script_path = util::get_script_path(py_script.unwrap())?;

    // Spawn the Python process
    let mut child = Command::new("python3")
        .arg(script_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write input to Python's stdin
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(input.as_bytes())?
    }

    // Wait for the process to finish and collect output
    let output = child.wait_with_output()?;

    // Return error if there is one
    let err = String::from_utf8_lossy(&output.stderr);
    if !err.trim().is_empty() {
        return Err(err.into());
    }

    // Return the output
    let output = String::from_utf8_lossy(&output.stdout);
    Ok(output.into_owned())
}
