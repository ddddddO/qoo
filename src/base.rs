pub trait Base {
    fn query(&self) -> String;

    // NOTE: 大変そう
    fn where_phrase(&self, where_str: &str) -> String {
        format!("{} where {}", self.query(), where_str)
    }

    fn to_sql(&self) -> String {
        self.query()
    }
}

pub trait SelectDeleteBase : Base {
    fn from_phrase(&self, table: &str) -> String {
        format!("{} from {}", self.query() , table)
    }
}