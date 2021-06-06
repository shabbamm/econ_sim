use crate::geography::*;
use crate::population::Community;
use serde::de;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub worlds: HashMap<usize, World>,
    pub continents: HashMap<usize, Continent>,
    pub regions: HashMap<usize, Region>,
    pub provinces: HashMap<usize, Province>,
    pub communities: HashMap<usize, Community>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            worlds: HashMap::new(),
            continents: HashMap::new(),
            regions: HashMap::new(),
            provinces: HashMap::new(),
            communities: HashMap::new(),
        }
    }

    pub fn init_from_config() -> GameState {
        let mut game_state = GameState::new();
        game_state.worlds = Config::load_config("config/worlds.json").unwrap();
        game_state.continents = Config::load_config("config/continents.json").unwrap();
        game_state.regions = Config::load_config("config/regions.json").unwrap();
        game_state.provinces = Config::load_config("config/provinces.json").unwrap();
        game_state.communities = Config::load_config("config/communities.json").unwrap();
        game_state
    }

    pub fn add_world(self: &mut Self, name: &str) {
        let id = self.worlds.len();
        self.worlds.insert(id, World::new(id, name.to_owned()));
    }

    pub fn add_continent(self: &mut Self, world_id: usize, name: &str) {
        let id = self.continents.len();
        self.continents
            .insert(id, Continent::new(world_id, id, name.to_owned()));
    }

    pub fn add_region(self: &mut Self, continent_id: usize, name: String) {
        let id = self.regions.len();
        self.regions.insert(id, Region::new(continent_id, id, name));
    }
    pub fn add_province(self: &mut Self, region_id: usize, name: String, resource_id: usize) {
        let id = self.provinces.len();
        self.provinces
            .insert(id, Province::new(region_id, id, name, resource_id));
    }
    pub fn add_community(self: &mut Self, province_id: usize, size: usize, money: isize) {
        let id = self.communities.len();
        self.communities
            .insert(id, Community::new(province_id, id, size, money));
    }

    pub fn cost_of_living(self: &mut Self) {
        for community in 0..self.communities.len() {
            self.communities.get_mut(&community).unwrap().money -= 10;
        }
    }
}

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
