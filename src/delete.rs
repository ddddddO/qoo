use crate::base::{Base, SelectDeleteBase, SelectUpdateDeleteBase};

pub struct DeleteBuilder {
    q: String
}

impl DeleteBuilder {
    pub fn delete() -> DeleteBuilder {
        DeleteBuilder {
            q: format!("delete"),
        }
    }

    pub fn from(mut self, table: &str) -> Self {
        self.q = self.from_phrase(table);
        self
    }

    pub fn wheres(mut self, where_str: &str) -> Self {
        self.q = self.where_phrase(where_str);
        self
    }
}

impl Base for DeleteBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl SelectDeleteBase for DeleteBuilder {}
impl SelectUpdateDeleteBase for DeleteBuilder {}