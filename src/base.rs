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