use crate::base::{Base};

pub struct InsertBuilder {
    q: String
}

impl InsertBuilder {
    pub fn insert(table: &str) -> InsertBuilder {
        InsertBuilder {
            q: format!("{} {}", "insert into", table)
        }
    }

    pub fn wheres(self, where_str: &str) -> Self {
        self.q = self.where_phrase(where_str);
        self
    }
}

impl base::Base for InsertBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}