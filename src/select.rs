use crate::base;

pub struct SelectBuilder {
    q: String
}

impl SelectBuilder {
    pub fn select(column: &str) -> SelectBuilder {
        SelectBuilder {
            q: format!("{} {}", "select", column),
        }
    }

    pub fn from(self, table: &str) -> SelectBuilder {
        SelectBuilder {
            q: format!("{} from {}", self.q , table),
        }
    }
}

impl base::Base for SelectBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}