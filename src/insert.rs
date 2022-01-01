use crate::base;

pub struct InsertBuilder {
    q: String
}

impl InsertBuilder {
    pub fn insert(table: &str) -> InsertBuilder {
        InsertBuilder {
            q: format!("{} {}", "insert into", table)
        }
    }
}

impl base::Base for InsertBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}