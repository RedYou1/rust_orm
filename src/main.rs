use crate::table::{Column, Queries, Table};
use const_format::formatcp;
use rust_query::Table;

pub mod column;
pub mod query;
pub mod table;

#[derive(Table)]
struct Test {
    nom: String,
    WOWADESRW: u32,
}

pub fn main() {
    println!(
        "{:?}",
        Test::columns()
            .iter()
            .map(Column::name)
            .collect::<Vec<&'static str>>()
    );
    println!("{}", Test::select_all().to_string());
}
