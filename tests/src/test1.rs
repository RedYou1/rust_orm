use mysql::{
    prelude::{FromRow, Queryable},
    FromRowError, PooledConn, Row,
};
use rust_query::table::{Column, Queries, Reference, Table};
use rust_query_mysql::mysqlrow::MySQLRow;

#[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
struct Test {
    db: i32,
    #[PrimaryKey]
    id: i32,
    nom: String,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
#[table_name("prof")]
struct Test2 {
    db: i32,
    #[PrimaryKey]
    id: i32,
    nom: Option<String>,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
#[table_name("test_prof")]
struct Test3 {
    #[PrimaryKey]
    db: i32,
    #[PrimaryKey]
    #[ForeignKey("db = db AND test_id = id")]
    test: Test,
    #[PrimaryKey]
    #[ForeignKey("db = db AND prof_id = id")]
    prof: Test2,
    score: i32,
}

pub fn test1(conn: &mut PooledConn) {
    println!("Testing 1");

    let mut select_all = Test3::select_all();
    let mut select_all = &mut select_all;
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![
            data(0),
            data(1),
            data(2),
            data(3),
            data(4),
            data(5),
            data(6),
            data(7)
        ]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), Test3::from_row)
            .err()
    );

    select_all = select_all.wheres("prof.id > 1".to_owned());
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(1), data(3), data(5), data(7)]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), Test3::from_row)
            .err()
    );

    select_all = select_all.orderbys("test_prof.db ASC, test_prof.score DESC".to_owned());
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(3), data(1), data(7), data(5)]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), Test3::from_row)
            .err()
    );

    select_all = select_all.limit(2);
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(3), data(1)]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), Test3::from_row)
            .err()
    );

    select_all = select_all.offset(1);
    assert_eq!(
        conn.query_map(select_all.to_string(), Test3::from_row).ok(),
        Some(vec![data(1), data(7)]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), Test3::from_row)
            .err()
    );
}

#[allow(clippy::too_many_lines)]
fn data(index: usize) -> Test3 {
    match index {
        0 => Test3 {
            db: 1,
            test: Test {
                db: 1,
                id: 1,
                nom: "A".to_owned(),
            },
            prof: Test2 {
                db: 1,
                id: 1,
                nom: None,
            },
            score: 1,
        },
        1 => Test3 {
            db: 1,
            test: Test {
                db: 1,
                id: 1,
                nom: "A".to_owned(),
            },
            prof: Test2 {
                db: 1,
                id: 2,
                nom: Some("B".to_owned()),
            },
            score: 2,
        },
        2 => Test3 {
            db: 1,
            test: Test {
                db: 1,
                id: 2,
                nom: "B".to_owned(),
            },
            prof: Test2 {
                db: 1,
                id: 1,
                nom: None,
            },
            score: 3,
        },
        3 => Test3 {
            db: 1,
            test: Test {
                db: 1,
                id: 2,
                nom: "B".to_owned(),
            },
            prof: Test2 {
                db: 1,
                id: 2,
                nom: Some("B".to_owned()),
            },
            score: 4,
        },
        4 => Test3 {
            db: 2,
            test: Test {
                db: 2,
                id: 1,
                nom: "A".to_owned(),
            },
            prof: Test2 {
                db: 2,
                id: 1,
                nom: None,
            },
            score: 1,
        },
        5 => Test3 {
            db: 2,
            test: Test {
                db: 2,
                id: 1,
                nom: "A".to_owned(),
            },
            prof: Test2 {
                db: 2,
                id: 2,
                nom: Some("B".to_owned()),
            },
            score: 2,
        },
        6 => Test3 {
            db: 2,
            test: Test {
                db: 2,
                id: 2,
                nom: "B".to_owned(),
            },
            prof: Test2 {
                db: 2,
                id: 1,
                nom: None,
            },
            score: 3,
        },
        7 => Test3 {
            db: 2,
            test: Test {
                db: 2,
                id: 2,
                nom: "B".to_owned(),
            },
            prof: Test2 {
                db: 2,
                id: 2,
                nom: Some("B".to_owned()),
            },
            score: 4,
        },
        _ => panic!("data not found"),
    }
}
