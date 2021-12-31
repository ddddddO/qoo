fn main() {
    let query = Query::select("col1".to_string()).from("table1".to_string());
    println!("{}", query.to_string());
}

struct Query {
    q: String
}

impl Query {
    fn select(column: String) -> Query {
        Query {
            q: format!("{} {}", "select", column),
        }
    }

    fn from(self, table: String) -> Query {
        Query {
            q: format!("{} from {}", self.q , table),
        }
    }

    fn to_string(self) -> String {
        self.q
    }
}