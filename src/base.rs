pub trait Base {
    fn query(&self) -> String;

    fn to_sql(&self) -> String {
        self.query()
    }
}

pub trait SelectDeleteBase : Base {
    fn from_phrase(&self, table: &str) -> String {
        format!("{} from {}", self.query() , table)
    }
}

pub trait SelectInsertBase : Base {
    fn clmns(&self, cs: &[&str]) -> String;
}

pub trait SelectUpdateDeleteBase : Base {
    // NOTE: 大変そう
    fn where_phrase(&self, where_str: &str) -> String {
        format!("{} where {}", self.query(), where_str)
    }
}