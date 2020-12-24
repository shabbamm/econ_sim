fn main() {
    let mut world_state = World::new();
    println!("{:?}", world_state);
}

#[derive(Debug)]
struct World {
    continents: Vec<Continent>,
    nations: Vec<Nation>,
    nation_count: usize,
}

impl World {
    fn new() -> World {
        World {
            continents: vec![Continent::new(1)],
            nations: vec![Nation::new()],
            nation_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Nation {
    state_religion: String,
    government: String,
    ruling_party: String,
    accepted_cultures: Vec<String>,
    political_reforms: Vec<String>,
    social_reforms: Vec<String>,
    economic_reforms: Vec<String>,
}

impl Nation {
    fn new() -> Nation {
        Nation {
            state_religion: String::new(),
            government: String::new(),
            ruling_party: String::new(),
            accepted_cultures: Vec::new(),
            political_reforms: Vec::new(),
            social_reforms: Vec::new(),
            economic_reforms: Vec::new(),
        }
    }

    fn update_state_religion(&mut self, input_string: &str) {
        self.state_religion = String::from(input_string);
    }

    fn update_government(&mut self, input_string: &str) {
        self.government = String::from(input_string);
    }

    fn update_ruling_party(&mut self, input_string: &str) {
        self.ruling_party = String::from(input_string);
    }

    fn update_accepted_cultures(&mut self, input_string: &str) {
        self.accepted_cultures.push(String::from(input_string));
    }
}

#[derive(Debug, Clone)]
struct Continent {
    regions: Vec<Region>,
    region_count: usize,
}

impl Continent {
    fn new(region_count: usize) -> Continent {
        Continent {
            regions: vec![Region::new(10); region_count],
            region_count: region_count,
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    id: usize,
    name: String,
    provinces: Vec<Provinces>,
    province_count: usize,
}

impl Region {
    fn new(province_count: usize) -> Region {
        Region {
            id: 0,
            name: String::new(),
            provinces: vec![Provinces::new(); 10],
            province_count: province_count,
        }
    }

    fn update_id(&mut self, input_integer: usize) {
        self.id = input_integer;
    }

    fn update_name(&mut self, input_string: &str) {
        self.name = String::from(input_string);
    }
}

#[derive(Debug, Clone)]
struct Provinces {
    owner: String,
    population_groups: Vec<Pop>,
    cores: Vec<Provinces>,
    goods: Vec<Goods>,
}

impl Provinces {
    fn new() -> Provinces {
        Provinces {
            cores: Vec::new(),
            goods: Vec::new(),
            owner: String::new(),
            population_groups: Vec::new(),
        }
    }

    fn update_owner(&mut self, input_string: &str) {
        self.owner = String::from(input_string);
    }
}

#[derive(Debug, Clone)]
struct Pop {
    size: usize,
    culture: String,
    religion: String,
    social_class: String,
    basic_needs: Vec<Goods>,
    daily_needs: Vec<Goods>,
    luxury_needs: Vec<Goods>,
}

impl Pop {
    fn new() -> Pop {
        Pop {
            size: 0,
            culture: String::new(),
            religion: String::new(),
            social_class: String::new(),
            basic_needs: Vec::new(),
            daily_needs: Vec::new(),
            luxury_needs: Vec::new(),
        }
    }

    fn update_size(&mut self, input_integer: usize) {
        self.size = input_integer;
    }

    fn update_culture(&mut self, input_string: &str) {
        self.culture = String::from(input_string);
    }

    fn update_social_class(&mut self, input_string: &str) {
        self.social_class = String::from(input_string);
    }
}

#[derive(Debug, Clone)]
struct Goods {
    quantity: usize,
    name: String,
}

impl Goods {
    fn new() -> Goods {
        Goods {
            quantity: 0,
            name: String::new(),
        }
    }

    fn update_quantity(&mut self, input_integer: usize) {
        self.quantity = input_integer;
    }

    fn update_name(&mut self, input_string: &str) {
        self.name = String::from(input_string);
    }
}
