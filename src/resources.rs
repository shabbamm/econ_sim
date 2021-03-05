use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub facility: String,
    pub cost: f64,
}

// consider making resources a global item that settlements reference for trade value
impl Resource {
    pub fn new() -> Resource {
        Resource {
            name: String::from("Salmon"),
            facility: String::from("Fishery"),
            cost: 0.0,
        }
    }
}

/*
consumer_goods = {
    paper = {
        cost = 3.4
        color = { 216 185 77 }
    }
    cattle = {
        cost = 2.0
        color = { 84 227 40 }
    }
    fish = {
        cost = 1.5
        color = { 128 98 73 }
    }
    fruit = {
        cost = 1.8
        color = { 255 0 0 }
    }
    grain = {
        cost = 2.2
        color = { 219 157 36 }
    }
}
*/
