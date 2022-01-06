use crate::base::{Base, SelectDeleteBase, SelectInsertBase, SelectUpdateDeleteBase};

/// Structure for select statement.
pub struct SelectBuilder {
    q: String
}

impl SelectBuilder {
    /// Assemble the select statement.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let select_query =
    ///     SelectBuilder::select(&["col0", "col1"])
    ///         .columns(&["col2", "col3"])
    ///         .from("table1")
    ///         .wheres("id >= 100");
    /// assert_eq!(
    ///     select_query.to_sql(),
    ///     "select col0, col1, col2, col3 from table1 where id >= 100".to_string()
    /// );
    /// ```
    pub fn select(columns: &[&str]) -> Self {
        let clms = assemble_columns_statemente(columns);

        SelectBuilder {
            q: format!("{} {}", "select", clms),
        }
    }

    pub fn columns(mut self, columns: &[&str]) -> Self {
        let q = &self.q;
        if SelectBuilder::exists_prev_columns(q) {
            self.q = format!("{},", self.q);
        }

        let clms = self.clmns(columns);
        self.q = format!("{} {}", self.q, clms);
        self
    }

    fn exists_prev_columns(q: &String) -> bool {
        q.split(" ").count() >= 2
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
    fn clmns(&self, columns: &[&str]) -> String {
        assemble_columns_statemente(columns)
    }
}

fn assemble_columns_statemente(columns: &[&str]) -> String {
    let mut clms: String = "".to_string();
    for c in columns {
        if columns.last() == Some(&c) {
            clms = format!("{}{}", clms, c.to_string());
            break;
        }
        clms = format!("{}{}, ", clms, c.to_string())
    }
    clms
}

impl SelectUpdateDeleteBase for SelectBuilder {}