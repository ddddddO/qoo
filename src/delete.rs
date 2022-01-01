use crate::base::{Base, SelectDeleteBase, SelectUpdateDeleteBase};

/// Structure for delete statement.
pub struct DeleteBuilder {
    q: String
}

impl DeleteBuilder {
    /// Assemble the delete statement.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// let delete_query =
    ///     DeleteBuilder::delete()
    ///         .from("test1")
    ///         .wheres("id >= 100");
    /// assert_eq!(
    ///     delete_query.to_sql(),
    ///     "delete from test1 where id >= 100".to_string()
    /// );
    /// ```
    pub fn delete() -> DeleteBuilder {
        DeleteBuilder {
            q: format!("delete"),
        }
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

impl Base for DeleteBuilder {
    fn query(&self) -> String {
        self.q.to_string()
    }
}

impl SelectDeleteBase for DeleteBuilder {}
impl SelectUpdateDeleteBase for DeleteBuilder {}