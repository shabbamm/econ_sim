struct Building {
    building_type: String,
    level: u8,
    resource_cost: Vec<Resources>,
    build_time: u16,
}

/*
aeroplane_factory = {
    type = factory
    on_completion = factory
    completion_size = 0.2
    max_level = 99
    goods_cost =
    {
        machine_parts = 200
        electric_gear = 600
        steel = 600
        cement = 600
    }
    time = 730
    visibility = yes
    onmap = no

    production_type = aeroplane_factory
    pop_build_factory = yes
    advanced_factory = yes
}
*/
