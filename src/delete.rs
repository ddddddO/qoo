use crate::base;

pub struct DeleteBuilder {
    q: String
}

impl DeleteBuilder {
    pub fn delete() -> DeleteBuilder {
        DeleteBuilder {
            q: format!("delete"),
        }
    }
}

impl base::Base for DeleteBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl base::SelectDeleteBase for DeleteBuilder {
    fn from(&self, table: &str) -> DeleteBuilder {
        DeleteBuilder {
            q: format!("{} from {}", self.q , table),
        }
    }
}