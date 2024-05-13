use rust_query::table::{Column, Queries, Table};

#[derive(Table)]
struct Test {
    #[PrimaryKey]
    id: i32,
    nom: String,
}

#[derive(Table)]
#[table_name("prof")]
struct Test2 {
    #[PrimaryKey]
    id: i32,
    nom: Option<String>,
}

#[derive(Table)]
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

pub fn main() {
    println!(
        "{:?}",
        Test3::columns()
            .iter()
            .map(Column::name)
            .collect::<Vec<&'static str>>()
    );
    println!("{}", Test3::select_all().to_string());
}
