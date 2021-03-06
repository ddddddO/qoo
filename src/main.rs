use qoo::prelude::*;

fn main() {
    let select_query =
        SelectBuilder::select(&["col0", "col1"])
            .columns(&["col2", "col3"])
            .from("table1")
            .wheres("id >= 100");
    assert_eq!(
        select_query.to_sql(),
        "select col0, col1, col2, col3 from table1 where id >= 100"
    );

    let insert_query =
        InsertBuilder::insert("test1")
            .columns(&["col0", "col1"])
            .value("'xxx'")
            .value("'yyy'");
    assert_eq!(
        insert_query.to_sql(),
        "insert into test1 (col0, col1) values ('xxx', 'yyy')"
    );

    let update_query =
        UpdateBuilder::update("test1")
            .set("col1='aaa'")
            .set("col2='bbb'")
            .wheres("id >= 11");
    assert_eq!(
        update_query.to_sql(),
        "update test1 set col1='aaa', col2='bbb' where id >= 11"
    );

    let delete_query =
        DeleteBuilder::delete()
            .from("test1")
            .wheres("id >= 100");
    assert_eq!(
        delete_query.to_sql(),
        "delete from test1 where id >= 100"
    );
}
