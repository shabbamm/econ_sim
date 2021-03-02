pub struct Party {
    name: String,
    ideology: String,
    economic_policy: String,
    trade_policy: String,
    religious_policy: String,
    citizenship_policy: String,
    war_policy: String,
}

pub struct Nation {
    id: u16,
    name: String
    political_parties: Vec<Party>,
    owned_settlements: Vec<u32>,

}
/*
party = {
    name = "ABU_conservative_2"
    start_date = 1892.1.1
    end_date = 1935.1.1

    ideology = conservative

    economic_policy = laissez_faire
    trade_policy = protectionism
    religious_policy = moralism
    citizenship_policy = residency
    war_policy = pacifism
}

capital = 1167
primary_culture = bedouin
religion = shiite
government = absolute_monarchy
plurality = 0.0
nationalvalue = nv_order
literacy = 0.3

ruling_party = ABU_conservative
upper_house = {
	fascist = 0
	liberal = 10
	conservative = 75
	reactionary = 15
	anarcho_liberal = 0
	socialist = 0
	communist = 0
}

# Starting Consciousness
consciousness = 0
nonstate_consciousness = 0

oob = "ABU_oob.txt"

slavery = no_slavery
upper_house_composition = appointed
vote_franschise = none_voting
public_meetings = no_meeting
press_rights = state_press
trade_unions = no_trade_unions
voting_system = jefferson_method
political_parties = underground_parties

wage_reform = no_minimum_wage
work_hours = no_work_hour_limit
safety_regulations = no_safety
health_care = no_health_care
unemployment_subsidies = no_subsidies
pensions = no_pensions 
school_reforms = no_schools

1861.1.1 = {
	oob = "/1861/ABU_oob.txt"
}

1861.1.1 = {
	#Military Reforms
	foreign_weapons = yes_foreign_weapons
}

*/
