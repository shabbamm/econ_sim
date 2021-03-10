use crate::population::*;
use crate::resources::*;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    id: u32,
    name: String,
}

impl World {
    pub fn new(id: u32, name: String) -> Self {
        World { id: id, name: name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Continent {
    pub id: u32,
    pub name: String,
}

impl Continent {
    pub fn new(id: u32, name: String) -> Self {
        Continent { id: id, name: name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub id: u32,
    pub name: String,
}

impl Region {
    pub fn new(id: u32, name: String) -> Self {
        Region { id: id, name: name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settlement {
    // we live in a settlment; bottom text
    id: u32,
    name: String,
    //is_sea: bool,
    //is_coastal: bool,
    //is_lake: bool,
    //climate: String,
    resource_id: u32,
    // owner
    // controller
    // cores
    // resource
    // life_rating
}

impl Settlement {
    pub fn new(id: u32, name: String, resource_id: u32) -> Settlement {
        Settlement {
            id: id,
            name: name,
            resource_id: resource_id,
        }
    }
}
