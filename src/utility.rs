use serde_json::{Result, Value};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

// file path input
// struct with populated data output

fn json_to_string(path: &Path) -> String {
    let contents = fs::read_to_string(path).unwrap();
    contents
}

pub fn string_to_serde_value(path: &Path) -> Result<()> {
    let data = &json_to_string(path)[..];

    let v: Value = serde_json::from_str(data)?;

    println!("{:?}", v);
    Ok(())
}
