# rusql
A rust library for using sql and reusing it.

*is heavily influenced by [yesql](https://github.com/krisajenkins/yesql) so many thanks @krisajenkins*

## TODO
You can integrate rusql into your project through the [releases on crates.io](https://crates.io/crates/rusql):
```toml
# Cargo.toml
[dependencies]
rusql = "0.1.0"
```

## Overview
You need to write SQL and you need to reuse it. You don't want to duplicate the queries all over the code. This lib is for you.

*This lib does not execute any sql in the DB.*

## Usage
The basic idea is that put all the sql code in their sql files and then this libs parse this files and get all the queries.
The sql file need to be with this format :

```sql
-- name: insert-person
INSERT INTO "person" (name, data) VALUES ($1, $2);

-- name: select-all-person
SELECT id, name, data FROM person;

```
Note the comment with the  **-- name: ** , it will be the key value for get each query.
Also is necessary at the end of the query the ";".


```rust

extern crate rusql;
extern crate postgres;

use rusql::Loader;
use postgres::{Connection, SslMode};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}


fn main() {
    let conn = Connection::connect("postgres://postgres:local@localhost", &SslMode::None).unwrap();

    let queries = Loader::get_queries_from("examples/postgre.sql").unwrap().queries;

    //Drop table
    conn.execute(queries.get("drop-table-person").unwrap(), &[]).unwrap();

    //Create table
    conn.execute(queries.get("create-table-person").unwrap(), &[]).unwrap();

    let me = Person {
        id: 0,
        name: "Manuel".to_string(),
        data: None
    };

    //Insert into table
    conn.execute(queries.get("insert-person").unwrap(),
                 &[&me.name, &me.data]).unwrap();

    //Select
    let stmt = conn.prepare(queries.get("select-all").unwrap()).unwrap();
    for row in stmt.query(&[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person id : {}, name: {}", person.id, person.name);
    }
}

```

See the full example [here](https://github.com/manute/rusql/tree/master/examples)

## License

Copyright © 2015 Manuel Alonso

MIT License

### Why not execute SQL this lib?
In rust there is not yet a general driver like *JDBC* or go's *database/sql* so I decide to abstract first the parser of sql files to use directly with the libs already exists for each DB.
