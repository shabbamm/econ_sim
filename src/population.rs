use crate::resources::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Community {
    pub id: u32,
    //pub culture: String,
    //pub religion: String,
    //pub strata: String,
    //pub location: String,
    pub size: u32,
    //pub money: i64,
    //pub literacy: f64,
    //pub militancy: f64,
    //pub conciousness: f64,
    //pub political_interest: f64,
    //pub social_interest: f64,
    //pub needs_satisfaction: f64,
}

impl Community {
    pub fn new(id: u32, size: u32) -> Community {
        Community { id: id, size: size }
    }
}

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

#Kabul (996000/249000 POPS)
1209 = {
    aristocrats = {
        culture = pashtun
        religion = sunni
        size = 2750
    }

    bureaucrats = {
        culture = pashtun
        religion = sunni
        size = 825
    }

    officers = {
        culture = pashtun
        religion = sunni
        size = 625
    }

    clergymen = {
        culture = pashtun
        religion = sunni
        size = 1375
    }

    artisans = {
        culture = pashtun
        religion = sunni
        size = 5400
    }

    soldiers = {
        culture = pashtun
        religion = sunni
        size = 11000
    }

    farmers = {
        culture = pashtun
        religion = sunni
        size = 113025
    }

    aristocrats = {
        culture = tajik
        religion = sunni
        size = 2000
    }

    officers = {
        culture = tajik
        religion = sunni
        size = 350
    }

    clergymen = {
        culture = tajik
        religion = sunni
        size = 975
    }

    artisans = {
        culture = tajik
        religion = sunni
        size = 4900
    }

    soldiers = {
        culture = tajik
        religion = sunni
        size = 1100
    }

    farmers = {
        culture = tajik
        religion = sunni
        size = 88775
    }

    aristocrats = {
        culture = hazara
        religion = shiite
        size = 250
    }

    clergymen = {
        culture = hazara
        religion = shiite
        size = 138
    }

    artisans = {
        culture = hazara
        religion = shiite
        size = 800
    }

    soldiers = {
        culture = hazara
        religion = shiite
        size = 150
    }

    farmers = {
        culture = hazara
        religion = shiite
        size = 14662
    }
}

promotion chances
*/

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

enum NeedType {
    EVERYDAY,
    LIFE,
    LUXURY,
}

pub struct Needs {
    material_needs: Vec<Resource>,
}

impl Needs {
    pub fn new() {}
}

/*
everyday_needs = {
    coal = 1
    paper = 10
    luxury_clothes = 3
    luxury_furniture = 3
    wine = 10
    tobacco = 10
    coffee = 5
}

life_needs = {
    cattle = 0.75
    wool =  1
    fish =  1
    fruit =  1
    grain =  2.5
}


luxury_needs = {
    opium = 10
    telephones = 10
    automobiles = 10
    aeroplanes = 5
    radio = 10
    fuel = 10
    ammunition = 1
    small_arms = 1
    clipper_convoy = 2
    steamer_convoy = 2
}
*/
