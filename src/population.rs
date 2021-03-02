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

pub struct Crime {
    name: String,
    // add effects, but that comes when i figure out how things work at a base level
}

/*
citizen_guard = {
    life_rating = -0.02
    local_rgo_output = -0.1
    icon = 2

    trigger = {
    }
}
*/

/*
//example of culture block in old files

bears = {
    leader = polar_bear
    unit = EuropeanGC
    polar_bears = {
        color = { 200 200 200}
        first_names = { Nanuk Ursus Isbjorn Grrrowr Bjorn Grrgrrr Rawr Bamse }

        last_names = { Beliy Medved Bjorn Rawrrorr Rrrrr Grrr Gyp Qoi Styrbjorn Blomstfjellet }
    }
    union = SCA
}
*/
