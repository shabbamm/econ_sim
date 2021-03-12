// use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

// file path input
// struct with populated data output

pub fn load_scenario_file(path: &Path) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
