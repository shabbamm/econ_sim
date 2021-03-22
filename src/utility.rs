use crate::geography::*;
use crate::population::*;
use serde::de;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Config;

impl Config {
    pub fn load_config<T: de::DeserializeOwned, P: AsRef<Path>>(
        path: P,
    ) -> Result<HashMap<usize, T>, Box<Error>> {
        let file = File::open(path).expect("JSON config file failed to open");

        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }
}
