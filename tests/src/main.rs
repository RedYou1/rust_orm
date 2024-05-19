mod connection;

use mysql::{
    prelude::{FromRow, Queryable},
    Error, FromRowError, Row,
};
use rust_query::table::{Column, Queries, Table};
use rust_query_mysql::mysqlrow::MySQLRow;

use crate::connection::get_conn;

#[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
struct Test {
    #[PrimaryKey]
    id: i32,
    nom: String,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
#[table_name("prof")]
struct Test2 {
    #[PrimaryKey]
    id: i32,
    nom: Option<String>,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
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

#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
pub fn main() -> Result<(), Error> {
    let mut conn = get_conn()?;
    println!("Testing");

    let mut select_all = Test3::select_all();
    let mut select_all = &mut select_all;
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(0), data(1), data(2), data(3)])
    );

    select_all = select_all.wheres("prof.id > 1".to_owned());
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(1), data(3)])
    );

    select_all = select_all.orderbys("test_prof.score DESC".to_owned());
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(3), data(1)])
    );

    select_all = select_all.limit(1);
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(3)])
    );

    select_all = select_all.offset(1);
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(1)])
    );

    println!("Success");
    Ok(())
}

fn data(index: usize) -> Test3 {
    match index {
        0 => Test3 {
            test: Test {
                id: 1,
                nom: "A".to_owned(),
            },
            prof: Test2 { id: 1, nom: None },
            score: 1,
        },
        1 => Test3 {
            test: Test {
                id: 1,
                nom: "A".to_owned(),
            },
            prof: Test2 {
                id: 2,
                nom: Some("B".to_owned()),
            },
            score: 2,
        },
        2 => Test3 {
            test: Test {
                id: 2,
                nom: "B".to_owned(),
            },
            prof: Test2 { id: 1, nom: None },
            score: 3,
        },
        3 => Test3 {
            test: Test {
                id: 2,
                nom: "B".to_owned(),
            },
            prof: Test2 {
                id: 2,
                nom: Some("B".to_owned()),
            },
            score: 4,
        },
        _ => panic!("data not found"),
    }
}
