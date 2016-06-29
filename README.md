
# rawsql
A rust library for *using* and *reusing* SQL.

[![Build Status](https://travis-ci.org/manute/rawsql.svg?branch=master)](https://travis-ci.org/manute/rawsql) [![Latest Version](https://img.shields.io/crates/v/rawsql.svg)](https://crates.io/crates/rawsql)

*is heavily influenced by [yesql](https://github.com/krisajenkins/yesql) (many thanks @krisajenkins)*

###[DOC](http://manute.github.io/rawsql/rawsql/index.html)###

You can integrate rawsql into your project through the [releases on crates.io](https://crates.io/crates/rawsql):
```toml
# Cargo.toml
[dependencies]
rawsql = "0.1.0"
```

## Overview
You need to write SQL and you need to reuse it. You don't want to duplicate the queries all over the code. This lib is for you.

*This lib does not execute any sql in the DB.*

## Usage
The basic idea is separate the sql from the code to put into sql files. With this you gain sql powers and the ability to write sql only once that runs on your DB (your dba could modify these files too)

The sql file need to be with this format :

```sql
-- name: insert-person
INSERT INTO "person" (name, data) VALUES ($1, $2);

-- name: select-persons
SELECT id, name, data FROM person;

```
Note the comment with the  **-- name: ** , it will be the key value for get each query.
Also is necessary at the end of the query ";".


```rust

extern crate rawsql;

use rawsql::Loader;


fn main() {

    let queries = Loader::get_queries_from("examples/postgre.sql").unwrap().queries;

    //Insert query
    let q_ìnsert = queries.get("insert-person").unwrap();

    println!("{}", q-insert);

    //Select query
    let q_select = queries.get("select-persons").unwrap();
    println!("{}", q-select);

}

```

See the full example [here](https://github.com/manute/rawsql/tree/master/examples)

## License

Copyright © 2015 Manuel Alonso

MIT License

### Why not execute SQL this lib?
In rust there is not yet a general driver like *JDBC* or go's *database/sql* so I decide to abstract first the parser of sql files to use directly with the libs already exists for each DB.
