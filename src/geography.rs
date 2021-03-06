use crate::population::*;
use crate::resources::*;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub continents: Vec<Continent>,
}

impl World {
    pub fn new() -> Self {
        let mut continents = Vec::new();
        let data = r#""#;
        // data for the names of the continents are here
        for continent in 0..1
        /* the 1 here should be the length of the list of continents, or for continent in [Vec of continents] */
        {
            continents
                .push(Continent::new(/*the name of the continent being created is passed here*/));
        }

        World {
            continents: continents,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Continent {
    pub id: u32,
    pub name: String,
    pub regions: Vec<Region>,
}

impl Continent {
    pub fn new(/* the name is passed to here from above */) -> Self {
        let mut regions = Vec::new();
        // the the data for the regions should be here, based on the name above
        for _region in 0..1
        /* for each region in [continent_name] add new region */
        {
            regions.push(Region::new(/* the name for each region is passed here to create */));
        }

        Continent {
            id: 0,
            name: String::from(""), /* this is set from the above value that is passed into the new() */
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
    pub fn new(/* the name is passed here from above */) -> Self {
        let mut settlements = Vec::new();

        for _settlement in 0..1
        /*the 1 should also be replaced based on how many settlements there are in a specific region*/
        {
            settlements.push(Settlement::new(/*here is where we dont just pass in the name of the settlement, but also other data from it*/));
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

        for _community in 0..1 {
            communities.push(Community::new());
        }

        Settlement {
            id: 0,
            name: String::new(),
            is_sea: false,
            is_coastal: false,
            is_lake: false,
            climate: String::from("Temperate"),
            communities: communities,
            resource: Resource::new(),
        }
    }
}
