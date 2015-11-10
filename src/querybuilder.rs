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
