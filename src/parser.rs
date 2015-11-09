use std::io::{Read, Result};
use std::fs::File;
use std::collections::HashMap;

static TAG_NAME: &'static str = "-- name:";

/// Struct query with the sql query and the number of params
pub struct Query {
    pub query: String,
    pub params: i32
}

impl Query {
    fn new(query: String) -> Query {
        Query {
            query: query.to_string(),
            params: Query::get_params(query.to_string()),
        }
    }

    ///Count the total params as '?'
    fn get_params(query: String) -> i32 {
        query.as_bytes().iter()
            .filter(|&b| *b == 63 )
            .fold(0, |acc, _| acc + 1 )
    }
}

pub fn parse_file(path: &str) -> Result<HashMap<String, Query>> {

    let data_file = try!(read_file(path));

    let mut name = String::new();
    let mut query = String::new();
    let mut queries: HashMap<String, Query> = HashMap::new();

    for line in data_file.lines() {
        if line.is_empty(){
            continue;
        }
        if line.starts_with(TAG_NAME) {
            name = line.replace(TAG_NAME,"").trim_left().to_string();
            query = "".to_string();
            continue;
        }
        if !name.is_empty() {
            query = query + " " + &line.trim_left().to_string();
        }
        if !query.is_empty() && line.ends_with(";") {
            queries.insert(name, Query::new(query));
            name  = "".to_string();
            query = "".to_string();
        }
    }

    Ok(queries)
}

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(&path));
    let mut data_file = String::new();
    try!(file.read_to_string(&mut data_file));
    Ok(data_file)
}
