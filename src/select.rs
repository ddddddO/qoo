use crate::base::{self, SelectDeleteBase};

pub struct SelectBuilder {
    q: String
}

impl SelectBuilder {
    pub fn select(columns: Vec<&str>) -> SelectBuilder {
        let mut clms: String = "".to_string();
        let cs = &columns;
        for c in cs {
            if cs.last() == Some(&c) {
                clms = format!("{}{}", clms, c.to_string());
                break;
            }
            clms = format!("{}{}, ", clms, c.to_string())
        }

        SelectBuilder {
            q: format!("{} {}", "select", clms),
        }
    }

    pub fn from(self, table: &str) -> SelectBuilder {
        SelectBuilder {
            q: self.from_phrase(table),
        }
    }
}

impl base::Base for SelectBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl base::SelectDeleteBase for SelectBuilder {}