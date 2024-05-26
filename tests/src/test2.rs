use mysql::{
    prelude::{FromRow, Queryable},
    FromRowError, PooledConn, Row,
};
use rust_query::table::{Column, Queries, Reference, Table};
use rust_query_mysql::mysqlrow::{MySQLRow, RowFlatten};

#[derive(Debug, Table, MySQLRow, PartialEq, Eq, Clone)]
#[table_name("user_info")]
struct UserInfo {
    #[PrimaryKey]
    id: i32,
    uuid: Option<String>,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq, Clone)]
#[table_name("category")]
struct Category {
    #[PrimaryKey]
    id: i32,
    nom: String,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq, Clone)]
#[table_name("user")]
struct User {
    #[PrimaryKey]
    id: i32,
    nom: String,
    #[ForeignKey("category_user ON id = user_id", "category_id = id")]
    categories: Vec<Category>,
    #[ForeignKey("category_pet ON category.id = category_id", "category_id = id")]
    pets: Vec<Pet>,
    #[ForeignKey("id = user_id")]
    info: UserInfo,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq, Clone)]
#[table_name("pet")]
struct Pet {
    #[PrimaryKey]
    id: i32,
    nom: String,
}

pub fn test2(conn: &mut PooledConn) {
    println!("Testing 2");

    let select_all = User::select_all();
    assert_eq!(
        conn.query_map(select_all.to_string(), User::from_row)
            .ok()
            .map(|row| row.row_flatten()),
        Some(vec![
            User {
                id: 1,
                nom: "A".to_owned(),
                categories: vec![
                    Category {
                        id: 1,
                        nom: "A".to_owned()
                    },
                    Category {
                        id: 2,
                        nom: "B".to_owned()
                    }
                ],
                info: UserInfo { id: 1, uuid: None },
                pets: vec![
                    Pet {
                        id: 1,
                        nom: "A".to_owned()
                    },
                    Pet {
                        id: 2,
                        nom: "B".to_owned()
                    },
                ],
            },
            User {
                id: 2,
                nom: "B".to_owned(),
                categories: vec![Category {
                    id: 1,
                    nom: "A".to_owned()
                }],
                info: UserInfo {
                    id: 2,
                    uuid: Some("B".to_owned())
                },
                pets: vec![Pet {
                    id: 1,
                    nom: "A".to_owned()
                }],
            },
        ]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), User::from_row).err()
    );
}