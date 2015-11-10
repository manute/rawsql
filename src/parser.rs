use std::io::{Read, Result};
use std::fs::File;
use std::collections::HashMap;
use querybuilder::Query;


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


pub fn load_queries_from(path: &str) -> Result<HashMap<String, Query>> {

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_when_file_not_exists() {
        let q = load_queries_from("tests/non_exist.sql");
        match q {
            Ok(r) => r,
            Err(why) => panic!(why)
        };
    }

    #[test]
    fn it_should_parse_queries() {
        let q = load_queries_from("tests/example.sql");

        let res = match q {
            Ok(r) => r,
            Err(why) => panic!(why)
        };

        let mut query = match res.get("simple")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(query.params , 1);
        assert_eq!(query.command , "query");

        query = match res.get("two-lines")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(query.params , 0);
        assert_eq!(query.command , "exec");

        query = match res.get("complex")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(query.params , 2);
        assert_eq!(query.command , "query");

    }
}
