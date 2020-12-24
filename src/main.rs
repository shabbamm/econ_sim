fn main() {
    let mut world_state = World::new();
    println!("{:?}", world_state);
}

#[derive(Debug)]
struct World {
    continents: Vec<Continent>,
    contintent_count: usize,
    nations: Vec<Nation>,
    nation_count: usize,
}

impl World {
    fn new() -> World {
        World {
            // TODO find a way to import the regioncount for each continent
            continents: Vec::new(),
            contintent_count: 0,
            // TODO find way to populate nations vector based on starting nation values
            nations: Vec::new(),
            // TODO increment and decrement based on nation creation/destruction
            nation_count: 0,
        }
    }

    fn create_continent() {}

    fn create_nation(
        name: String,
        state_religion: String,
        government_type: String,
        ruling_party: String,
        accepted_cultures: Vec<String>,
        political_reforms: Vec<String>,
        social_reforms: Vec<String>,
        economic_reforms: Vec<String>,
        owned_provinces: Vec<Province>,
        core_provinces: Vec<Province>,
    ) -> Nation {
        Nation {
            name: name,
            state_religion: state_religion,
            government_type: government_type,
            ruling_party: ruling_party,
            accepted_cultures: accepted_cultures,
            political_reforms: political_reforms,
            social_reforms: social_reforms,
            economic_reforms: economic_reforms,
            owned_provinces: owned_provinces,
            core_provinces: core_provinces,
        }
    }

    //String::from("Austria"),
    //String::from("Catholocism"),
    //String::from("Democracy"),
    //String::from("Conservative"),
    //vec![String::from("Austrian")],
    //vec![], // TODO political reforms on creation
    //vec![], // TODO social reforms on creation
    //vec![], // TODO economic reforms on creation
    //vec![], // TODO owned provinces on creation (should probobly be a reference)
    //vec![], // TODO core provinces on creation (should also be a vector of references)
}

#[derive(Debug, Clone)]
struct Nation {
    name: String,
    state_religion: String,
    government_type: String,
    ruling_party: String,
    accepted_cultures: Vec<String>,
    political_reforms: Vec<String>,
    social_reforms: Vec<String>,
    economic_reforms: Vec<String>,
    owned_provinces: Vec<Province>,
    core_provinces: Vec<Province>,
}

impl Nation {
    fn update_state_religion(&mut self, input_string: &str) {
        self.state_religion = String::from(input_string);
    }

    fn update_government_type(&mut self, input_string: &str) {
        self.government_type = String::from(input_string);
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
    name: String,
    regions: Vec<Region>,
    region_count: usize,
}

impl Continent {
    fn new(name: String, region_count: usize) -> Continent {
        Continent {
            name: name,
            regions: vec![Region::new(1); region_count],
            region_count: region_count,
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    id: usize,
    name: String,
    provinces: Vec<Province>,
    province_count: usize,
}

impl Region {
    fn new(province_count: usize) -> Region {
        Region {
            id: 0,
            name: String::new(),
            provinces: vec![Province::new(); 1],
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
struct Province {
    owner: String,
    population_groups: Vec<Pop>,
    cores: Vec<Province>,
    goods: Vec<Goods>,
}

impl Province {
    fn new() -> Province {
        Province {
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
