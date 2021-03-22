mod geography;
mod population;
mod resources;
mod utility;

use geography::*;
use population::Community;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use utility::LoadConfig;

fn main() {
    let mut game_state = GameState::new();

    game_state.worlds = LoadConfig::load_worlds(Path::new("config/geography/worlds.json")).unwrap();

    for world in game_state.worlds {}

    game_state.continents =
        LoadConfig::load_continents(Path::new("config/geography/continents.json")).unwrap();

    for continent in game_state.continents {
        println!("{:?}", continent);
    }

    //println!("{:?}", game_state);
}

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    worlds: HashMap<u32, World>,
    continents: HashMap<u32, Continent>,
    regions: HashMap<u32, Region>,
    provinces: HashMap<u32, Province>,
    communities: HashMap<u32, Community>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            worlds: HashMap::new(),
            continents: HashMap::new(),
            regions: HashMap::new(),
            provinces: HashMap::new(),
            communities: HashMap::new(),
        }
    }

    fn new_world(self: &mut Self, id: u32, name: String) {
        self.worlds.insert(id, World::new(id, name));
    }

    fn new_continent(self: &mut Self, id: u32, name: String) {
        self.continents.insert(id, Continent::new(id, name));
    }

    fn new_region(self: &mut Self, id: u32, name: String) {
        self.regions.insert(id, Region::new(id, name));
    }
    fn new_province(self: &mut Self, id: u32, name: String, resource_id: u32) {
        self.provinces
            .insert(id, Province::new(id, name, resource_id));
    }
    fn new_community(self: &mut Self, id: u32, size: u32) {
        self.communities.insert(id, Community::new(id, size));
    }
}
