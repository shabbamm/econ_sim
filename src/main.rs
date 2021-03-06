mod geography;
mod population;
mod resources;
mod utility;

use serde::{Deserialize, Serialize};

fn main() {
    let mut world = geography::World::new();
    println!("{:#?}", world);
    //for continent in &world_state.continents {
    //    println!("{:#?}", continent);
    //}
}

#[derive(Debug, Serialize, Deserialize)]
struct GameState {}

/*8
match age {
    Some(age) => {if 21 allowed to drink}
    None => {age not valid}
}
*/

// unit name lists??

// match statement for New / Load game
