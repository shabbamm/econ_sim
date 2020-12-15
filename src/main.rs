use std::collections::HashMap;

fn main() {
    let mut world = World::new();

    world.add_region(0, "Austria");

    let regions = world.region_map.get_mut(&mut 0).unwrap();

    regions.add_county(0, "Linz");
    regions.add_county(1, "Vienna");
    regions.add_county(2, "Salzburg");

    println!("{:?}", world);
}

#[derive(Debug)]
struct World {
    region_map: HashMap<u8, Region>,
}

impl World {
    fn new() -> World {
        World {
            region_map: HashMap::new(),
        }
    }

    fn add_region(&mut self, region_id: u8, region_name: &str) {
        self.region_map
            .insert(region_id, Region::new(region_id, region_name));
    }
}

#[derive(Debug)]
struct Region {
    region_id: u8,
    name: String,
    county_map: HashMap<u8, County>,
}

impl Region {
    fn new(region_id: u8, region_name: &str) -> Region {
        Region {
            region_id: region_id,
            name: region_name.to_string(),
            county_map: HashMap::new(),
        }
    }

    fn add_county(&mut self, county_id: u8, county_name: &str) {
        self.county_map
            .insert(county_id, County::new(county_id, county_name));
    }
}

#[derive(Debug)]
struct Nation {
    nation_id: u8,
    name: String,
    treasury: i128,
}

impl Nation {
    fn new(nation_id: u8, name: &str, treasury: i128) -> Nation {
        Nation {
            nation_id: nation_id,
            name: name.to_string(),
            treasury: treasury,
        }
    }
}

#[derive(Debug)]
struct County {
    county_id: u8,
    name: String,
}

impl County {
    fn new(county_id: u8, input_name: &str) -> County {
        County {
            county_id: county_id,
            name: input_name.to_string(),
        }
    }
}
