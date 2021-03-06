mod geography;
mod population;
mod resources;
mod utility;

use geography::*;
use std::collections::HashMap;

fn main() {}

struct GameState {
    world: HashMap<u32, World>,
    contintents: HashMap<u32, Continent>,
    regions: HashMap<u32, Region>,
    settlement: HashMap<u32, Settlement>,
}

impl GameState {}
