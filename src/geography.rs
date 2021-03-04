use crate::population::*;
use crate::resources::*;
use crate::utility::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub continents: Vec<String>,
}

impl World {
    pub fn new() -> Result<World> {
        let continents_data = r#"{
            "continents": [
                "Africa",
                "Antartica",
                "Asia",
                "Australia",
                "Europe",
                "NorthAmerica",
                "SouthAmerica"
            ]
        }"#;

        let world: World = serde_json::from_str(continents_data)?;

        Ok(world)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    // owner
    // controller
    // cores
    // resource
    // life_rating
}

impl Settlement {
    pub fn new() -> Settlement {
        let mut communities = Vec::new();

        for _community in 0..50 {
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
