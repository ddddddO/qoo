fn main() {
    let select_query = SelectBuilder::select("col1").from("table1");
    println!("{}", select_query.to_sql());

    let delete_query = DeleteBuilder::delete().from("test1");
    println!("{}", delete_query.to_sql());
}

trait Base {
    fn query(&self) -> String;

    fn to_sql(&self) -> String {
        self.query()
    }
}

struct SelectBuilder {
    q: String
}

impl SelectBuilder {
    fn select(column: &str) -> SelectBuilder {
        SelectBuilder {
            q: format!("{} {}", "select", column),
        }
    }

    fn from(self, table: &str) -> SelectBuilder {
        SelectBuilder {
            q: format!("{} from {}", self.q , table),
        }
    }
}

impl Base for SelectBuilder {
    fn query(&self) -> String {
        self.q.to_string()
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

    fn from(self, table: &str) -> DeleteBuilder {
        DeleteBuilder {
            q: format!("{} from {}", self.q , table),
        }
    }
}

impl Base for DeleteBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

struct InsertBuilder {
    q: String
}

impl InsertBuilder {
    fn insert(table: &str) -> InsertBuilder {
        InsertBuilder {
            q: format!("{} {}", "insert into", table)
        }
    }
}

impl Base for InsertBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}