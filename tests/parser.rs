extern crate rusql;

use rusql::parser::parse_file;


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
fn it_should_parse_queries() {
    let q = parse_file("tests/example.sql");

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
fn it_should_parse_queries_with_count_params() {
    let q = parse_file("tests/example.sql");

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
