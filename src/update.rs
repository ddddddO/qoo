use crate::base::{Base};

pub struct UpdateBuilder {
    cnt: u32,
    q: String
}

impl UpdateBuilder {
    pub fn update(table: &str) -> Self {
        UpdateBuilder {
            cnt: 0,
            q: format!("update {}", table.to_string()),
        }
    }

    pub fn set(mut self, val: &str) -> Self {
        if self.cnt == 0 {
            self.q = format!("{} set {}", self.q, val);
            self.cnt += self.cnt + 1;
        } else {
            self.q = format!("{}, {}", self.q, val);
            self.cnt += self.cnt + 1;
        }
        self
    }

    pub fn wheres(mut self, where_str: &str) -> Self {
        self.q = self.where_phrase(where_str);
        self
    }
}

impl Base for UpdateBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}
