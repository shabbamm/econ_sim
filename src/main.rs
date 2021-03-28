mod geography;
mod population;
mod resources;
mod utility;

use std::{thread, time};

fn main() {
    let mut game_state = utility::GameState::init_from_config();

    let framerate = time::Duration::from_secs(1);
    while &game_state.communities.len() > &0 {
        game_state.cost_of_living();

        println!("{:?}", game_state.communities);

        thread::sleep(framerate);
    }
}
