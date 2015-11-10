extern crate rusql;

use rusql::parser::load_queries_from;


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

    match res.get("simple")  {
        Some(_) => assert!(true),
        None => assert!(false)
    };

    match res.get("two-lines")  {
        Some(_) => assert!(true),
        None => assert!(false)
    };

    match res.get("complex")  {
        Some(_) => assert!(true),
        None => assert!(false)
    };
}

#[test]
fn it_should_parse_queries_and_get_his_attributes() {
    let q = load_queries_from("tests/example.sql");

    let res = match q {
        Ok(r) => r,
        Err(why) => panic!(why)
    };

    let s = match res.get("simple")  {
        Some(r) => r,
        None => panic!("no result on get query")
    };

    assert_eq!(s.params , 1);
    assert_eq!(s.command , "query");

    let c = match res.get("complex")  {
        Some(r) => r,
        None => panic!("no result on get query")
    };

    assert_eq!(c.params , 2);
    assert_eq!(c.command , "query");

}
