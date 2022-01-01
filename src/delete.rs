use crate::base::{Base, SelectDeleteBase};

pub struct DeleteBuilder {
    q: String
}

impl DeleteBuilder {
    pub fn delete() -> DeleteBuilder {
        DeleteBuilder {
            q: format!("delete"),
        }
    }

    pub fn from(self, table: &str) -> DeleteBuilder {
        DeleteBuilder {
            q: self.from_phrase(table),
        }
    }

    pub fn wheres(self, where_str: &str) -> DeleteBuilder {
        DeleteBuilder {
            q: self.where_phrase(where_str),
        }
    }
}

impl Base for DeleteBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl SelectDeleteBase for DeleteBuilder {}