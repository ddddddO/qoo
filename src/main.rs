fn main() {
    let select_query = SelectBuilder::select("col1".to_string()).from("table1".to_string());
    println!("{}", select_query.to_string());

    let delete_query = DeleteBuilder::delete().from("test1".to_string());
    println!("{}", delete_query.to_string());
}

trait Base {
    fn to_string(self) -> String;
}

struct SelectBuilder {
    q: String
}

impl SelectBuilder {
    fn select(column: String) -> SelectBuilder {
        SelectBuilder {
            q: format!("{} {}", "select", column),
        }
    }

    fn from(self, table: String) -> SelectBuilder {
        SelectBuilder {
            q: format!("{} from {}", self.q , table),
        }
    }
}

impl Base for SelectBuilder {
    fn to_string(self) -> String {
        self.q
    }
}

struct DeleteBuilder {
    q: String
}

impl DeleteBuilder {
    fn delete() -> DeleteBuilder {
        DeleteBuilder {
            q: format!("delete"),
        }
    }

    fn from(self, table: String) -> DeleteBuilder {
        DeleteBuilder {
            q: format!("{} from {}", self.q , table),
        }
    }
}

impl Base for DeleteBuilder {
    fn to_string(self) -> String {
        self.q
    }
}

struct InsertBuilder {
    q: String
}

impl InsertBuilder {
    fn insert(table: String) -> InsertBuilder {
        InsertBuilder {
            q: format!("{} {}", "insert into", table)
        }
    }
}