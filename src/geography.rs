use crate::population::*;
use crate::resources::*;

#[derive(Debug)]
pub struct Settlement {
    // we live in a society; bottom text
    id: u32,
    name: String,
    is_sea: bool,
    is_coastal: bool,
    is_lake: bool,
    climate: String,
    communities: Vec<Community>,
    resource: Resource,
}

impl Settlement {
    pub fn new() -> Settlement {
        let mut communities = Vec::new();

        for community in 0..250 {
            communities.push(Community::new());
        }

        Settlement {
            id: 0,
            name: String::new(),
            is_sea: false,
            is_coastal: false,
            is_lake: false,
            climate: String::from(
                "this is super temporary and no climate is going to be this long",
            ),
            communities: communities,
            resource: Resource::new(),
        }
    }

    pub fn add_community(&mut self) {
        self.communities.push(Community::new());
    }
}

#[derive(Debug)]
pub struct Region {
    pub id: u32,
    pub name: String,
    pub settlements: Vec<Settlement>,
}

impl Region {
    pub fn new() -> Self {
        let mut settlements = Vec::new();

        for _settlement in 0..10 {
            settlements.push(Settlement::new());
        }

        Region {
            id: 0,
            name: String::new(),
            settlements: settlements,
        }
    }
}

#[derive(Debug)]
pub struct Continent {
    pub id: u32,
    pub name: String,
    pub regions: Vec<Region>,
}

impl Continent {
    pub fn new(name: &str) -> Self {
        let mut regions = Vec::new();

        for _region in 0..50 {
            regions.push(Region::new());
        }

        Continent {
            id: 0,
            name: String::from(name),
            regions: regions,
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub continents: Vec<Continent>,
}

impl World {
    pub fn new() -> Self {
        World {
            continents: vec![
                Continent::new("Africa"),
                Continent::new("Antartica"),
                Continent::new("Asia"),
                Continent::new("Australia"),
                Continent::new("Europe"),
                Continent::new("North America"),
                Continent::new("South America"),
            ],
        }
    }
}
