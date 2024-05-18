mod connection;

use mysql::{
    prelude::{FromRow, Queryable},
    Error, FromRowError, Row,
};
use rust_query::table::{Column, Queries, Table};
use rust_query_mysql::mysqlrow::MySQLRow;

use crate::connection::get_conn;

#[derive(Debug, Table, MySQLRow)]
struct Test {
    #[PrimaryKey]
    id: i32,
    nom: String,
}

#[derive(Debug, Table, MySQLRow)]
#[table_name("prof")]
struct Test2 {
    #[PrimaryKey]
    id: i32,
    nom: Option<String>,
}

#[derive(Debug, Table, MySQLRow)]
#[table_name("test_prof")]
struct Test3 {
    #[PrimaryKey]
    #[ForeignKey]
    test: Test,
    #[PrimaryKey]
    #[ForeignKey]
    prof: Test2,
    score: i32,
}

#[allow(clippy::missing_errors_doc)]
pub fn main() -> Result<(), Error> {
    println!(
        "{:?}",
        Test3::columns()
            .iter()
            .map(Column::name)
            .collect::<Vec<&'static str>>()
    );
    println!("{}", Test3::select_all().to_string());

    let mut conn = get_conn()?;

    let data = conn.query_map(Test3::select_all().to_string(), Test3::from_row)?;

    for data in data {
        println!("{data:?}");
    }

    Ok(())
}
