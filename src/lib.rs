use std::io::{Read,  Result};
use std::fs::File;
use std::collections::HashMap;

static TAG_NAME: &'static str = "-- name :";

pub fn parse_file(path: &str) -> Result<HashMap<String, String>> {

    let mut file = try!(File::open(&path));
    let mut data_file = String::new();
    try!(file.read_to_string(&mut data_file));

    let lines: Vec<String> = data_file.lines().map(|l| l.trim().to_string()).collect();

    let names: Vec<String> = lines.clone().into_iter()
        .filter(|l| l.starts_with(TAG_NAME))
        .map(|l| l.replace(TAG_NAME,"").trim_left().to_string())
        .collect();

    let queries: Vec<String> = lines.clone().into_iter()
        .filter(|l| !l.starts_with(TAG_NAME))
        .map(|l| l.trim_left().to_string())
        .collect();

    let result: HashMap<String, String> = names.into_iter().zip(queries).collect();

    return Ok(result);
}
