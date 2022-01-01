use crate::base::{Base};

pub struct UpdateBuilder {
    cnt: u32,
    q: String
}

impl UpdateBuilder {
    pub fn update(table: &str) -> UpdateBuilder {
        UpdateBuilder {
            cnt: 0,
            q: format!("update {}", table.to_string()),
        }
    }

    pub fn set(self, val: &str) -> UpdateBuilder {
        let mut q = "".to_string();
        let mut cnt = 0;
        if self.cnt == 0 {
            q = format!("{} set {}", self.q, val);
            cnt += self.cnt + 1;
        } else {
            q = format!("{}, {}", self.q, val);
            cnt += self.cnt + 1;
        }

        UpdateBuilder {
            cnt: cnt,
            q: q,
        }
    }

    pub fn wheres(self, where_str: &str) -> UpdateBuilder {
        UpdateBuilder {
            cnt: self.cnt,
            q: self.where_phrase(where_str),
        }
    }
}

impl Base for UpdateBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}
