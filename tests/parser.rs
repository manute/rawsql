extern crate rusql;

use rusql::parse_file;


#[test]
#[should_panic]
fn it_when_file_not_exists() {
    let q = parse_file("tests/non_exist.sql");
    match q {
        Ok(r) => r,
        Err(why) => panic!(why)
    };
}

#[test]
fn it_should_parse_simple_query() {
    let q = parse_file("tests/example.sql");

    let res = match q {
        Ok(r) => r,
        Err(why) => panic!(why)
    };

    match res.get("get-names")  {
        Some(_) => assert!(true),
        None => assert!(false)
    };
}
