fn main() {
    let query = Query::select("test".to_string());
    println!("{}", query.to_string());
}

struct Query {
    q: String
}

impl Query {
    fn select(table: String) -> Query {
        Query{
            q: format!("{} {}", "select", table),
        }
    }

    fn to_string(self) -> String {
        self.q
    }
}