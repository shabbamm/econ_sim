fn main() {
    println!("the fun never ends");
    let nation = Nation::new();
    let continent = Continent::new();
    let region = Region::new();
    let provinces = Provinces::new();
    let pop = Pop::new();
    let goods = Goods::new();

    println!("{:?}", nation);
    println!("{:?}", continent);
    println!("{:?}", region);
    println!("{:?}", provinces);
    println!("{:?}", pop);
    println!("{:?}", goods);
}

#[derive(Debug)]
struct Nation {
    accepted_cultures: Vec<String>,
    state_religion: String,
    government: String,
    ruling_party: String,
    political_reforms: Vec<String>,
    social_reforms: Vec<String>,
    economic_reforms: Vec<String>,
}

impl Nation {
    fn new() -> Nation {
        Nation {
            accepted_cultures: Vec::new(),
            state_religion: String::new(),
            government: String::new(),
            ruling_party: String::new(),
            political_reforms: Vec::new(),
            social_reforms: Vec::new(),
            economic_reforms: Vec::new(),
        }
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
    name: String,
    id: u8,
    province_vector: Vec<Provinces>,
}

impl Region {
    fn new() -> Region {
        Region {
            name: String::new(),
            id: 0,
            province_vector: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Provinces {
    population_groups: Vec<Pop>,
    owner: String,
    cores: Vec<Provinces>,
    goods: Vec<Goods>,
}

impl Provinces {
    fn new() -> Provinces {
        Provinces {
            population_groups: Vec::new(),
            owner: String::new(),
            cores: Vec::new(),
            goods: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Pop {
    culture: String, //enum
    religion: String,
    social_class: String,
    size: u128,
    basic_needs: Vec<Goods>,
    daily_needs: Vec<Goods>,
    luxury_needs: Vec<Goods>,
}

impl Pop {
    fn new() -> Pop {
        Pop {
            culture: String::new(),
            religion: String::new(),
            social_class: String::new(),
            size: 0,
            basic_needs: Vec::new(),
            daily_needs: Vec::new(),
            luxury_needs: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Goods {
    name: String,
    quantity: u128,
}

impl Goods {
    fn new() -> Goods {
        Goods {
            name: String::new(),
            quantity: 0,
        }
    }
}
