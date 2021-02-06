use crate::resources::*;

#[derive(Debug)]
pub struct Community {
    pub culture: String,
    pub religion: String,
    pub strata: String,
    pub location: String,
    pub size: u32,
    pub money: i64,
    pub literacy: f32,
    pub militancy: f32,
    pub conciousness: f32,
    pub political_interest: f32,
    pub social_interest: f32,
    pub needs_satisfaction: f32,
}

impl Community {
    pub fn new() -> Community {
        Community {
            culture: String::from(""),
            religion: String::from(""),
            strata: String::from(""),
            location: String::from(""),
            size: 0,
            money: 0,
            literacy: 0.0,
            militancy: 0.0,
            conciousness: 0.0,
            social_interest: 0.0,
            political_interest: 0.0,
            needs_satisfaction: 0.0,
        }
    }

    pub fn add_size(&mut self, quantity: u32) {
        self.size += quantity;
    }

    pub fn sub_size(&mut self, quantity: u32) {
        self.size -= quantity;
    }

    pub fn add_money(&mut self, quantity: i64) {
        self.money += quantity;
    }

    pub fn sub_money(&mut self, quantity: i64) {
        self.money -= quantity;
    }
}

#[derive(Debug)]
pub struct Settlement {
    // we live in a society; bottom text
    pub society: Vec<Community>,
    pub resource: Resource,
}

impl Settlement {
    pub fn new() -> Settlement {
        Settlement {
            society: Vec::new(),
            resource: Resource::new(),
        }
    }

    pub fn add_community(&mut self) {
        self.society.push(Community::new());
    }
}
