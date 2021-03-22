mod geography;
mod population;
mod resources;
mod utility;

use geography::*;
use population::Community;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utility::LoadConfig;

fn main() {
    let mut game_state = GameState::new();

    game_state.worlds = LoadConfig::load_worlds("config/worlds.json").unwrap();
    game_state.continents = LoadConfig::load_continents("config/continents.json").unwrap();
    game_state.regions = LoadConfig::load_region("config/regions.json").unwrap();
    game_state.provinces = LoadConfig::load_province("config/provinces.json").unwrap();
    game_state.communities = LoadConfig::load_communities("config/communities.json").unwrap();

    for world in game_state.worlds {
        println!("{:?}", world);
    }
    for continent in game_state.continents {
        println!("{:?}", continent);
    }
    for region in game_state.regions {
        println!("{:?}", region);
    }
    for province in game_state.provinces {
        println!("{:?}", province);
    }
    for community in game_state.communities {
        println!("{:?}", community);
    }
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
