mod geography;
mod population;
mod resources;
mod utility;

use rand::Rng;

fn main() {
    let mut game_state = utility::GameState::init_from_config();
    let mut rng = rand::thread_rng();

    println!("{:?}", game_state);

    //while &game_state.communities.len() > &0 {}
}
