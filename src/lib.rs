use std::io::{Read,  Result};
use std::fs::File;
use std::collections::HashMap;

static TAG_NAME: &'static str = "-- name :";

pub fn parse_file(path: &str) -> Result<HashMap<String, String>> {

    let mut file = try!(File::open(&path));
    let mut data_file = String::new();
    try!(file.read_to_string(&mut data_file));

    let mut result = HashMap::new();
    let mut name = String::new();
    let mut query = String::new();

    for line in data_file.lines() {
        if line.is_empty(){
            continue;
        }
        if line.starts_with(TAG_NAME) {
            name = line.replace(TAG_NAME,"").trim_left().to_string();
            continue;
        }
        if !name.is_empty() {
            query = query + " " + &line.trim_left().to_string();
        }
        if !query.is_empty() && line.ends_with(";") {
            result.insert(name, query);
            name  = "".to_string();
            query = "".to_string();
        }
    }

    return Ok(result);
}
