mod geography;
mod population;
mod resources;
mod utility;

fn main() {
    let mut world_state = geography::World::new().unwrap();

    //for continent in &world_state.continents {
    //    println!("{:#?}", continent);
    //}
    println!("{:#?}", world_state);
}

/*
match age {
    Some(age) => {if 21 allowed to drink}
    None => {age not valid}
}
*/

// unit name lists??

// match statement for New / Load game
