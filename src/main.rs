use qoo::base::*;
use qoo::select::*;
use qoo::delete::*;

fn main() {
    let select_query = SelectBuilder::select("col1").from("table1");
    println!("{}", select_query.to_sql());

    let delete_query = DeleteBuilder::delete().from("test1");
    println!("{}", delete_query.to_sql());
}
