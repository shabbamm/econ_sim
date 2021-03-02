use crate::resources::*;

#[derive(Debug)]
pub struct Community {
    pub id: u32,
    pub culture: String,
    pub religion: String,
    pub strata: String,
    pub location: String,
    pub size: u32,
    pub money: i64,
    pub literacy: f64,
    pub militancy: f64,
    pub conciousness: f64,
    pub political_interest: f64,
    pub social_interest: f64,
    pub needs_satisfaction: f64,
}

impl Community {
    pub fn new() -> Community {
        Community {
            id: 0,
            culture: String::from("this is a sample text that shouldnt be here but is here to test the bounds of memory"),
            religion: String::from("this is a sample text that shouldnt be here but is here to test the bounds of memory"),
            strata: String::from("this is a sample text that shouldnt be here but is here to test the bounds of memory"),
            location: String::from("this is a sample text that shouldnt be here but is here to test the bounds of memory"),
            size: 1,
            money: 1,
            literacy: 1.0,
            militancy: 1.0,
            conciousness: 1.0,
            social_interest: 1.0,
            political_interest: 1.0,
            needs_satisfaction: 1.0,
        }
    }

    pub fn update_size(&mut self, quantity: u32) {
        self.size += quantity;
    }

    pub fn update_money(&mut self, quantity: i64) {
        self.money += quantity;
    }
}

#[derive(Debug)]
pub struct Settlement {
    // we live in a society; bottom text
    pub id: u32,
    pub name: String,
    pub communities: Vec<Community>,
    pub resource: Resource,
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
