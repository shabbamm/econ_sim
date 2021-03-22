use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    id: u32,
    name: String,
}

impl World {
    pub fn new(id: u32, name: String) -> Self {
        World { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Continent {
    pub id: u32,
    pub name: String,
}

impl Continent {
    pub fn new(id: u32, name: String) -> Self {
        Continent { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: u32,
    pub name: String,
}

impl Region {
    pub fn new(id: u32, name: String) -> Self {
        Region { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Province {
    // we live in a province; bottom text
    id: u32,
    name: String,
    resource_id: u32,
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
    pub fn new(id: u32, name: String, resource_id: u32) -> Province {
        Province {
            id,
            name,
            resource_id,
        }
    }
}
