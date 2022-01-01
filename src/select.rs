use crate::base::{Base, SelectDeleteBase, SelectInsertBase, SelectUpdateDeleteBase};

pub struct SelectBuilder {
    q: String
}

impl SelectBuilder {
    pub fn select(columns: Vec<&str>) -> Self {
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

    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        let exists_prev_clmn = self.q.split(" ").count();
        if exists_prev_clmn >= 2 {
            self.q = format!("{},", self.q);
        }

        let clms = self.clmns(columns);
        self.q = format!("{} {}", self.q, clms);
        self
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

impl Base for SelectBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl SelectDeleteBase for SelectBuilder {}

impl SelectInsertBase for SelectBuilder {
    fn clmns(&self, columns: Vec<&str>) -> String {
        let mut clms: String = "".to_string();
        let cs = &columns;
        for c in cs {
            if cs.last() == Some(&c) {
                clms = format!("{}{}", clms, c.to_string());
                break;
            }
            clms = format!("{}{}, ", clms, c.to_string())
        }
        clms
    }
}

impl SelectUpdateDeleteBase for SelectBuilder {}