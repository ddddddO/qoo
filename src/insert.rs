use crate::base::{Base, SelectInsertBase};

/// Structure for insert statement.
pub struct InsertBuilder {
    cnt: u32,
    q: String
}

impl InsertBuilder {
    /// Assemble the insert statement.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let insert_query =
    ///     InsertBuilder::insert("test1")
    ///         .columns(["col0", "col1"].to_vec())
    ///         .value("'xxx'")
    ///         .value("'yyy'");
    /// assert_eq!(
    ///     insert_query.to_sql(),
    ///     "insert into test1 (col0, col1) values ('xxx', 'yyy')"
    /// );
    /// ```
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
