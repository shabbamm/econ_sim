// use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

// file path input
// struct with populated data output

pub fn load_file_to_buffer(path: &Path) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    println!("{:?}", file);
    let buffered = BufReader::new(file);
    println!("{:?}", buffered);
    Ok(buffered)
}

pub fn deserialize_buffer_to_struct() {}
