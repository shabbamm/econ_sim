mod geography;
mod population;
mod resources;

fn main() {
    let mut world_state = geography::World::new();

    println!("{:?}", world_state);

    //for continent in &world_state.continents {
    //    println!("{:#?}", continent);
    //}
}

/*8
match age {
    Some(age) => {if 21 allowed to drink}
    None => {age not valid}
}
*/

// unit name lists??

// match statement for New / Load game
