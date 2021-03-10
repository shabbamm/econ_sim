mod geography;
mod population;
mod resources;
mod utility;

use geography::*;
use population::Community;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    let mut game_state = GameState::new();

    game_state.create_new_world(0, String::from("Earth"));

    for continent in 0..1 {
        game_state.create_new_continent(continent, String::from("Africa"));
    }

    for region in 0..1 {
        game_state.create_new_region(region, String::from("Congo"));
    }

    for settlement in 0..1 {
        game_state.create_new_settlement(settlement, String::from("Kinshasa"), 0);
    }

    for community in 0..1 {
        game_state.create_new_community(community, 999);
    }
    //add worlds
    //add continents
    //add regions
    //etc
}

#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    worlds: HashMap<u32, World>,
    continents: HashMap<u32, Continent>,
    regions: HashMap<u32, Region>,
    settlements: HashMap<u32, Settlement>,
    communities: HashMap<u32, Community>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            worlds: HashMap::new(),
            continents: HashMap::new(),
            regions: HashMap::new(),
            settlements: HashMap::new(),
            communities: HashMap::new(),
        }
    }

    fn create_new_world(self: &mut Self, id: u32, name: String) {
        self.worlds.insert(id, World::new(id, name));
    }

    fn create_new_continent(self: &mut Self, id: u32, name: String) {
        self.continents.insert(id, Continent::new(id, name));
    }

    fn create_new_region(self: &mut Self, id: u32, name: String) {
        self.regions.insert(id, Region::new(id, name));
    }
    fn create_new_settlement(self: &mut Self, id: u32, name: String, resource_id: u32) {
        self.settlements
            .insert(id, Settlement::new(id, name, resource_id));
    }
    fn create_new_community(self: &mut Self, id: u32, size: u32) {
        self.communities.insert(id, Community::new(id, size));
    }
}
