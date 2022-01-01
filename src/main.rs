use qoo::base::*;
use qoo::select::*;
use qoo::delete::*;

fn main() {
    // TODO: .to_vec()無くしたい
    let select_query = SelectBuilder::select(["col0", "col1", "col2"].to_vec()).from("table1");
    println!("{}", select_query.to_sql());

    let delete_query = DeleteBuilder::delete().from("test1");
    println!("{}", delete_query.to_sql());
}
