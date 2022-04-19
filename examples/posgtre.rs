extern crate postgres;
extern crate rawsql;

use postgres::{Connection, SslMode};
use rawsql::Loader;

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::connect("postgres://postgres:local@localhost", &SslMode::None).unwrap();

    let queries = Loader::get_queries_from("examples/postgre.sql").unwrap();

    // Drop table
    conn.execute(queries.get("drop-table-person").unwrap(), &[])
        .unwrap();

    // Create table
    conn.execute(queries.get("create-table-person").unwrap(), &[])
        .unwrap();

    let me = Person {
        id: 0,
        name: "Manuel".to_string(),
        data: None,
    };

    // Insert into table
    conn.execute(queries.get("insert-person").unwrap(), &[&me.name, &me.data])
        .unwrap();

    // Select
    let stmt = conn.prepare(queries.get("select-all").unwrap()).unwrap();
    for row in stmt.query(&[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2),
        };
        println!("Found person id : {}, name: {}", person.id, person.name);
    }
}
