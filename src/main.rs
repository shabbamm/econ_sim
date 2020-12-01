use std::collections::HashMap;

fn main() {
    let mut world_state = World::new();
    println!("{:?}", world_state);

    world_state.regions.insert(0, Region::new("Austria"));
    world_state.regions.insert(1, Region::new("France"));
    world_state.regions.insert(2, Region::new("Vietnam"));

    println!("{:?}", world_state.regions);

    let mut austria = world_state.regions.get(&0).unwrap(); 

    austria.counties.insert(0, County::new("Vienna"));

    println!("{:?}", austria);

    
}

#[derive(Debug)]
struct World {
    regions: HashMap<u8, Region>,
}

impl World {
    fn new() -> World {
        World {
            regions: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Region {
    name: String,
    counties: HashMap<u8, County>,
}

impl Region {
    fn new(input_name: &str) -> Region {
        Region {
            name: input_name.to_string(),
            counties: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct County {
    name: String,
}

impl County {
    fn new(input_name: &str) -> County {
        County {
            name: input_name.to_string(),
        }
    }
}