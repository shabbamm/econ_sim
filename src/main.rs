mod geography;
mod population;
mod resources;
mod utility;

use geography::*;
use population::Community;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utility::Config;

fn main() {
    let mut game_state = GameState::init_from_config();

    let result = game_state.communities.contains_key(&0);

    for num in 0..1 {
        game_state.new_community(game_state.communities.len(), 348 / 13 * num + 1);
    }

    println!("{:#?}", game_state);
}

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    worlds: HashMap<usize, World>,
    continents: HashMap<usize, Continent>,
    regions: HashMap<usize, Region>,
    provinces: HashMap<usize, Province>,
    communities: HashMap<usize, Community>,
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

    fn init_from_config() -> GameState {
        let mut game_state = GameState::new();
        game_state.worlds = Config::load_config("config/worlds.json").unwrap();
        game_state.continents = Config::load_config("config/continents.json").unwrap();
        game_state.regions = Config::load_config("config/regions.json").unwrap();
        game_state.provinces = Config::load_config("config/provinces.json").unwrap();
        game_state.communities = Config::load_config("config/communities.json").unwrap();
        game_state
    }

    fn new_world(self: &mut Self, id: usize, name: String) {
        self.worlds.insert(id, World::new(id, name));
    }

    fn new_continent(self: &mut Self, id: usize, name: String) {
        self.continents.insert(id, Continent::new(id, name));
    }

    fn new_region(self: &mut Self, id: usize, name: String) {
        self.regions.insert(id, Region::new(id, name));
    }
    fn new_province(self: &mut Self, id: usize, name: String, resource_id: usize) {
        self.provinces
            .insert(id, Province::new(id, name, resource_id));
    }
    fn new_community(self: &mut Self, id: usize, size: usize) {
        self.communities.insert(id, Community::new(id, size));
    }
}
