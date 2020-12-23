fn main() {
    let mut nation = Nation::new();
    let mut continent = Continent::new();
    let mut region = Region::new();
    let mut provinces = Provinces::new();
    let mut pop = Pop::new();
    let mut goods = Goods::new();

    println!("\n{:?}\n", nation);
    nation.update_state_religion("Atheism");
    nation.update_government("Democracy");
    nation.update_ruling_party("Socialists");
    nation.update_accepted_cultures("Austrian");
    nation.update_accepted_cultures("German");
    nation.update_accepted_cultures("Czech");
    println!("\n{:?}\n", nation);
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Continent {
    regions: Vec<Region>,
}

impl Continent {
    fn new() -> Continent {
        Continent {
            regions: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Region {
    id: u8,
    name: String,
    province_vector: Vec<Provinces>,
}

impl Region {
    fn new() -> Region {
        Region {
            id: 0,
            name: String::new(),
            province_vector: Vec::new(),
        }
    }

    fn update_id(&mut self, input_integer: u8) {
        self.id = input_integer;
    }

    fn update_name(&mut self, input_string: &str) {
        self.name = String::from(input_string);
    }
}

#[derive(Debug)]
struct Provinces {
    owner: String,
    population_groups: Vec<Pop>,
    cores: Vec<Provinces>,
    goods: Vec<Goods>,
}

impl Provinces {
    fn new() -> Provinces {
        Provinces {
            owner: String::new(),
            population_groups: Vec::new(),
            cores: Vec::new(),
            goods: Vec::new(),
        }
    }

    fn update_owner(&mut self, input_string: &str) {
        self.owner = String::from(input_string);
    }
}

#[derive(Debug)]
struct Pop {
    size: u128,
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

    fn update_size(&mut self, input_integer: u128) {
        self.size = input_integer;
    }

    fn update_culture(&mut self, input_string: &str) {
        self.culture = String::from(input_string);
    }

    fn update_social_class(&mut self, input_string: &str) {
        self.social_class = String::from(input_string);
    }
}

#[derive(Debug)]
struct Goods {
    quantity: u128,
    name: String,
}

impl Goods {
    fn new() -> Goods {
        Goods {
            quantity: 0,
            name: String::new(),
        }
    }

    fn update_quantity(&mut self, input_integer: u128) {
        self.quantity = input_integer;
    }

    fn update_name(&mut self, input_string: &str) {
        self.name = String::from(input_string);
    }
}
