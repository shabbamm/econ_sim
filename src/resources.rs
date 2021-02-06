#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub facility: String,
}

// consider making resources a global item that settlements reference for trade value
impl Resource {
    pub fn new() -> Resource {
        Resource {
            name: String::from(""),
            facility: String::from(""),
        }
    }
}
