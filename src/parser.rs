use std::io::{Read, Result};
use std::fs::File;
use std::collections::HashMap;

/// Struct query with the sql query and the number of params
pub struct Query {
    pub query: String,
    pub params: i32,
    pub command: String
}

impl Query {
    pub fn new(query: String, command: String) -> Query {
        Query {
            query: query.to_string(),
            params: Query::get_params(query),
            command: command
        }
    }

    ///Count the total params as '?'
    fn get_params(query: String) -> i32 {
        query.as_bytes().iter()
            .filter(|&b| *b == 63 )
            .fold(0, |acc, _| acc + 1 )
    }
}


struct Parser {
    name: String,
    query: String,
    command: String
}

impl Parser {
    fn init() -> Parser {
        Parser { name: String::new(), query: String::new(), command: String::new()}
    }

    fn tag_for(&mut self,line: &str, what: &str)  {
        let tag = line.replace("--","").replace(what,"").replace(":","").trim_left().to_string();
        if what == "name" {
            self.name = tag.to_string();
            self.query = "".to_string();
            self.command = "".to_string();
        }
        if what == "command" {
            self.command = tag ;
        }

    }
    fn can_save_query(&mut self) -> bool {
        !self.name.is_empty() && !self.command.is_empty()
    }
}

pub fn parse_file(path: &str) -> Result<HashMap<String, Query>> {

    let data_file = try!(read_file(path));

    let mut parser = Parser::init();
    let mut queries: HashMap<String, Query> = HashMap::new();

    for line in data_file.lines() {
        if line.is_empty(){
            continue;
        }
        if is_tagged_for(line, "name") {
            parser.tag_for(line, "name");
            continue;
        }
        if is_tagged_for(line, "command") {
            parser.tag_for(line, "command");
            continue;
        }
        if parser.can_save_query(){
            parser.query = parser.query + " " + &line.trim_left().to_string();
        }
        if !parser.query.is_empty() && line.ends_with(";") {
            queries.insert(parser.name, Query::new(parser.query, parser.command));
            parser = Parser::init()
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

fn is_tagged_for(line: &str, what: &str) -> bool {
    line.starts_with("--") && line.contains(what)
}
