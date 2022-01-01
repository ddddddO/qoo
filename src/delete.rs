use crate::base::{self, SelectDeleteBase};

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
}

impl base::Base for DeleteBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl base::SelectDeleteBase for DeleteBuilder {}