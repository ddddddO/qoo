use qoo::base::*;
use qoo::select::*;
use qoo::delete::*;

fn main() {
    // TODO: .to_vec()無くしたい
    let select_query = SelectBuilder::select(["col0", "col1", "col2"].to_vec()).from("table1").wheres("id >= 100");
    assert_eq!(
        select_query.to_sql(),
        "select col0, col1, col2 from table1 where id >= 100".to_string()
    );

    let delete_query = DeleteBuilder::delete().from("test1").wheres("id >= 100");
    assert_eq!(
        delete_query.to_sql(),
        "delete from test1 where id >= 100".to_string()
    );
}
