fn main() {
    let mut world_state = World::new();
    world_state.add_continent(String::from("Afroeurasia"));
    world_state.add_continent(String::from("Antartica"));
    world_state.add_continent(String::from("America"));
    world_state.add_continent(String::from("Australia"));

    for continent in 0..4 {
        for region in 0..4 {
            world_state.continents[continent].add_region(region + 1, (region + 1).to_string());
            for province in 0..1 {
                world_state.continents[continent].regions[region]
                    .add_province(province.to_string());
                for group in 0..1 {
                    world_state.continents[continent].regions[region].provinces[province]
                        .add_pop_group(
                            100000000,
                            group.to_string(),
                            group.to_string(),
                            group.to_string(),
                            group.to_string(),
                        );
                }
                for cores in 0..4 {}
                for goods in 0..4 {}
            }
        }
    }

    for nation in 1..3 {
        world_state.add_nation(
            nation.to_string(),
            nation.to_string(),
            nation.to_string(),
            nation.to_string(),
        );
    }
    println!("\n{:#?}", world_state);
}

#[derive(Debug)]
struct World {
    continents: Vec<Continent>,
    nations: Vec<Nation>,
}

impl World {
    fn new() -> World {
        World {
            // TODO find a way to import the regioncount for each continent
            continents: Vec::new(),
            // TODO find way to populate nations vector based on starting nation values
            nations: Vec::new(),
            // TODO increment and decrement based on nation creation/destruction
        }
    }

    fn add_continent(&mut self, name: String) {
        self.continents.push(Continent::new(name));
    }

    fn add_nation(
        &mut self,
        name: String,
        state_religion: String,
        government_type: String,
        ruling_party: String,
    ) {
        self.nations.push(Nation::new(
            name,
            state_religion,
            government_type,
            ruling_party,
        ));
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
    owned_provinces: Vec<usize>,
    core_provinces: Vec<usize>,
}

impl Nation {
    fn new(
        name: String,
        state_religion: String,
        government_type: String,
        ruling_party: String,
    ) -> Nation {
        Nation {
            name: name,
            state_religion: state_religion,
            government_type: government_type,
            ruling_party: ruling_party,
            accepted_cultures: Vec::new(),
            political_reforms: Vec::new(),
            social_reforms: Vec::new(),
            economic_reforms: Vec::new(),
            owned_provinces: Vec::new(),
            core_provinces: Vec::new(),
        }
    }

    fn update_name(&mut self, input_string: &str) {
        self.name = String::from(input_string);
    }

    fn update_state_religion(&mut self, input_string: &str) {
        self.state_religion = String::from(input_string);
    }

    fn update_government_type(&mut self, input_string: &str) {
        self.government_type = String::from(input_string);
    }

    fn update_ruling_party(&mut self, input_string: &str) {
        self.ruling_party = String::from(input_string);
    }

    fn add_accepted_culture(&mut self, input_string: &str) {
        self.accepted_cultures.push(String::from(input_string));
    }

    fn add_political_reform(&mut self, input_string: &str) {
        self.political_reforms.push(String::from(input_string));
    }

    fn add_social_reform(&mut self, input_string: &str) {
        self.social_reforms.push(String::from(input_string));
    }

    fn add_economic_reform(&mut self, input_string: &str) {
        self.economic_reforms.push(String::from(input_string));
    }

    fn add_owned_province(&mut self, province_id: usize) {
        self.owned_provinces.push(province_id);
    }
}

#[derive(Debug, Clone)]
struct Continent {
    name: String,
    regions: Vec<Region>,
}

impl Continent {
    fn new(name: String) -> Continent {
        Continent {
            name: name,
            regions: Vec::new(),
        }
    }

    fn add_region(&mut self, id: usize, name: String) {
        self.regions.push(Region::new(id, name));
    }
}

#[derive(Debug, Clone)]
struct Region {
    id: usize,
    name: String,
    provinces: Vec<Province>,
}

impl Region {
    fn new(id: usize, name: String) -> Region {
        Region {
            id: id,
            name: name,
            provinces: Vec::new(),
        }
    }

    fn add_province(&mut self, owner: String) {
        self.provinces.push(Province::new(owner));
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
    fn new(owner: String) -> Province {
        Province {
            owner: owner,
            cores: Vec::new(),
            goods: Vec::new(),
            population_groups: Vec::new(),
        }
    }

    fn add_pop_group(
        &mut self,
        size: usize,
        culture: String,
        religion: String,
        social_class: String,
        basic_needs: String,
    ) {
        self.population_groups
            .push(Pop::new(size, culture, religion, social_class, basic_needs));
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
    fn new(
        size: usize,
        culture: String,
        religion: String,
        social_class: String,
        basic_needs: String,
    ) -> Pop {
        Pop {
            size: size,
            culture: culture,
            religion: religion,
            social_class: social_class,
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
