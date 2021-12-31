fn main() {
    let select_query = Query::select("col1".to_string()).from("table1".to_string());
    println!("{}", select_query.to_string());

    let delete_query = Query::delete().from("test1".to_string());
    println!("{}", delete_query.to_string());
}

trait Base {
    fn to_string(self) -> String;
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

    fn delete() -> Query {
        Query {
            q: format!("delete"),
        }
    }

    fn from(self, table: String) -> Query {
        Query {
            q: format!("{} from {}", self.q , table),
        }
    }
}

impl Base for Query {
    fn to_string(self) -> String {
        self.q
    }
}