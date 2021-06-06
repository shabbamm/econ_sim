use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    id: usize,
    name: String,
}

impl World {
    pub fn new(id: usize, name: String) -> Self {
        World { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Continent {
    pub world_id: usize,
    pub id: usize,
    pub name: String,
}

impl Continent {
    pub fn new(world_id: usize, id: usize, name: String) -> Self {
        Continent { world_id, id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub continent_id: usize,
    pub id: usize,
    pub name: String,
}

impl Region {
    pub fn new(continent_id: usize, id: usize, name: String) -> Self {
        Region {
            continent_id,
            id,
            name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Province {
    // we live in a province; bottom text
    region_id: usize,
    id: usize,
    name: String,
    resource_id: usize,
    //is_sea: bool,
    //is_coastal: bool,
    //is_lake: bool,
    //climate: String,
    // owner
    // controller
    // cores
    // resource
    // life_rating
}

impl Province {
    pub fn new(region_id: usize, id: usize, name: String, resource_id: usize) -> Province {
        Province {
            region_id,
            id,
            name,
            resource_id,
        }
    }
}
