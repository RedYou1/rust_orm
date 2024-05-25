use mysql::{
    prelude::{FromRow, Queryable},
    FromRowError, PooledConn, Row,
};
use rust_query::table::{Column, Queries, Reference, Table};
use rust_query_mysql::mysqlrow::{MySQLRow, RowFlatten};

// #[derive(Debug, Table, MySQLRow, PartialEq, Eq)]
// #[table_name("user_info")]
// struct UserInfo {
//     #[PrimaryKey]
//     id: i32,
//     #[ForeignKey]
//     user: User,
//     uuid: String,
// }

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
    #[ForeignKey("id", "user_id")]
    categories: Vec<CategoryUser>,
}

#[derive(Debug, Table, MySQLRow, PartialEq, Eq, Clone)]
#[table_name("category_user")]
struct CategoryUser {
    #[PrimaryKey]
    user_id: i32,
    #[PrimaryKey]
    #[ForeignKey("category_id", "id")]
    category: Category,
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
                    CategoryUser {
                        user_id: 1,
                        category: Category {
                            id: 1,
                            nom: "A".to_owned()
                        }
                    },
                    CategoryUser {
                        user_id: 1,
                        category: Category {
                            id: 2,
                            nom: "B".to_owned()
                        }
                    }
                ]
            },
            User {
                id: 2,
                nom: "B".to_owned(),
                categories: vec![CategoryUser {
                    user_id: 2,
                    category: Category {
                        id: 1,
                        nom: "A".to_owned()
                    }
                }]
            },
        ]),
        "Failed with:{:?} for {:?}",
        select_all.to_string(),
        conn.query_map(select_all.to_string(), User::from_row).err()
    );
}
