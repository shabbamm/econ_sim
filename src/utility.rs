use crate::geography::*;
use crate::population::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct LoadConfig;

impl LoadConfig {
    pub fn load_worlds<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, World>, Box<Error>> {
        let file = File::open(path).expect("JSON config file failed to open");

        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }
    pub fn load_continents<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, Continent>, Box<Error>> {
        let file = File::open(path).expect("JSON config file failed to open");

        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }
    pub fn load_region<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, Region>, Box<Error>> {
        let file = File::open(path).expect("JSON config file failed to open");

        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }
    pub fn load_province<P: AsRef<Path>>(path: P) -> Result<HashMap<u32, Province>, Box<Error>> {
        let file = File::open(path).expect("JSON config file failed to open");

        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }
    pub fn load_communities<P: AsRef<Path>>(
        path: P,
    ) -> Result<HashMap<u32, Community>, Box<Error>> {
        let file = File::open(path).expect("JSON config file failed to open");

        let reader = BufReader::new(file);

        let data = serde_json::from_reader(reader)?;

        Ok(data)
    }
}
