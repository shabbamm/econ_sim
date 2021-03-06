mod geography;
mod population;
mod resources;
mod utility;

use geography::*;
use population::Community;
use std::collections::HashMap;

fn main() {
    let game_state = GameState::new();
    println!("{:#?}", game_state);
}

#[derive(Debug)]
struct GameState {
    world: HashMap<u32, World>,
    contintents: HashMap<u32, Continent>,
    regions: HashMap<u32, Region>,
    settlements: HashMap<u32, Settlement>,
    communities: HashMap<u32, Community>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            world: HashMap::new(),
            contintents: HashMap::new(),
            regions: HashMap::new(),
            settlements: HashMap::new(),
            communities: HashMap::new(),
        }
    }
}
