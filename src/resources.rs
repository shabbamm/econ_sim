#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub facility: String,
}

// consider making resources a global item that settlements reference for trade value
impl Resource {
    pub fn new() -> Resource {
        Resource {
            name: String::from("this is a sample text that shouldnt be here but is here to test the bounds of memory"),
            facility: String::from("this is a sample text that shouldnt be here but is here to test the bounds of memory"),
        }
    }
}
