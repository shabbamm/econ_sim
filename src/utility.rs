use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub fn open_file(data: &str) -> Result<()> {
    //let path = "config/geography.json";
    //
    //let input = File::open(path)?;
    //let buffered = BufReader::new(input);
    //
    //for line in buffered.lines() {
    //    println!("{}", line?);
    //}

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let john: Person = serde_json::from_str(data)?;

    println!("{}", john.name);

    Ok(())
}
