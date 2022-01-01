use crate::base::{Base, SelectInsertBase};

pub struct InsertBuilder {
    cnt: u32,
    q: String
}

impl InsertBuilder {
    pub fn insert(table: &str) -> InsertBuilder {
        InsertBuilder {
            cnt: 0,
            q: format!("{} {}", "insert into", table)
        }
    }

    pub fn columns(mut self, columns: Vec<&str>) -> Self {
        let clms = self.clmns(columns);
        self.q = format!("{} {} ", self.q, clms);
        self
    }

    pub fn value(mut self, val: &str) -> Self {
        if self.cnt == 0 {
            self.q = format!("{}values ({}", self.q, val);
            self.cnt += self.cnt + 1;
        } else {
            self.q = format!("{}, {}", self.q, val);
            self.cnt += self.cnt + 1;
        }
        self
    }
}

impl Base for InsertBuilder {
    fn query(&self) -> String {
        format!("{})", self.q.to_string())
    }
}

impl SelectInsertBase for InsertBuilder {
    fn clmns(&self, columns: Vec<&str>) -> String {
        let mut clms: String = "(".to_string();
        let cs = &columns;
        for c in cs {
            if cs.last() == Some(&c) {
                clms = format!("{}{})", clms, c.to_string());
                break;
            }
            clms = format!("{}{}, ", clms, c.to_string())
        }
        clms
    }
}
