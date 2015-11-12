use std::io::{Read, Result};
use std::fs::File;
use std::collections::HashMap;


struct Parser {
    name: String,
    query: String,
}

impl Parser {
    fn init() -> Parser {
        Parser { name: String::new(), query: String::new()}
    }

    fn tag_name(&mut self, line: &str)  {
        let tag = line.replace("--","").replace("name","").replace(":","").trim_left().to_string();
        self.name = tag.to_string();
        self.query = "".to_string();
    }
    fn build_query(&mut self, line: &str) {
        let q = line.trim_left();
        if self.query.is_empty(){
            self.query = q.to_string();
        }else{
            self.query = self.query.to_string() + " " + &q;
        }
    }

    fn is_starting_query(&mut self) -> bool {
        !self.name.is_empty()
    }

    fn is_tagged_name(&mut self, line: &str) -> bool {
        line.starts_with("--") && line.contains("name")
    }
}

pub struct Loader {
    pub queries: HashMap<String, String>
}

impl Loader {
    pub fn get_queries_from(path: &str) -> Result<Loader> {

        let data_file = try!(read_file(path));

        let mut parser = Parser::init();
        let mut queries: HashMap<String, String> = HashMap::new();

        for line in data_file.lines() {
            if line.is_empty(){
                continue;
            }
            if parser.is_tagged_name(line) {
                parser.tag_name(line);
                continue;
            }
            if parser.is_starting_query(){
                parser.build_query(line);
            }
            if !parser.query.is_empty() && line.ends_with(";") {
                queries.insert(parser.name, parser.query);
                parser = Parser::init()
            }
        }
        Ok(Loader{queries: queries})
    }
}

fn read_file(path: &str) -> Result<String> {
    let mut file = try!(File::open(&path));
    let mut data_file = String::new();
    try!(file.read_to_string(&mut data_file));
    Ok(data_file)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_when_file_not_exists() {
        let loaded = Loader::get_queries_from("tests/non_exist.sql");
        match loaded {
            Ok(r) => r,
            Err(why) => panic!(why)
        };
    }

    #[test]
    fn it_should_parse_queries() {
        let loaded = Loader::get_queries_from("tests/example.sql");

        let res = match loaded {
            Ok(r) => r,
            Err(why) => panic!(why)
        };

        let q_simple = match res.queries.get("simple")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(q_simple , "SELECT * FROM table1 u where  u.name = ?;");

        let q_2_lines = match res.queries.get("two-lines")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(q_2_lines , "Insert INTO table2 SELECT * FROM table1;");

        let q_complex = match res.queries.get("complex")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(q_complex , "SELECT * FROM Customers c INNER JOIN CustomerAccounts ca ON ca.CustomerID = c.CustomerID AND c.State = ? INNER JOIN Accounts a ON ca.AccountID = a.AccountID AND a.Status = ?;");

        let q_psql = match res.queries.get("psql-insert")  {
            Some(r) => r,
            None => panic!("no result on get query")
        };

        assert_eq!(q_psql , "INSERT INTO person (name, data) VALUES ($1, $2);");
    }
}
