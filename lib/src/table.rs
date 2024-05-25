use std::collections::HashMap;

pub use crate::column::Column;
use crate::query::select::Select;
pub use rust_query_macro::Table;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReferenceJoin {
    OneOrNone = 1,
    ExaclyOne = 2,
    Multiple = 4,
    MultipleOption = 8,
    MultipleEmpty = 16,
    MultipleOptionEmpty = 32,
}
impl From<u8> for ReferenceJoin {
    fn from(value: u8) -> Self {
        match value {
            1 => ReferenceJoin::OneOrNone,
            2 => ReferenceJoin::ExaclyOne,
            4 => ReferenceJoin::Multiple,
            8 => ReferenceJoin::MultipleOption,
            16 => ReferenceJoin::MultipleEmpty,
            32 => ReferenceJoin::MultipleOptionEmpty,
            _ => panic!("ReferenceJoin from u8 not found"),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Reference {
    pub join: ReferenceJoin,
    pub from_table_name: &'static str,
    pub to_table_name: &'static str,
    pub identifiers: Vec<String>,
    pub all_columns: HashMap<&'static str, Vec<Column>>,
    pub references: Vec<Reference>,
}

pub trait Table {
    fn identifiers() -> Vec<Column>;
    fn id_eq(&self, b: &Self) -> bool;
    fn table_name() -> &'static str;
    fn columns() -> Vec<Column>;
    fn references() -> Vec<Reference>;
}

pub trait Queries {
    fn all_columns() -> HashMap<&'static str, Vec<Column>>;
    fn select_all() -> Select;
}

impl<T: Table> Queries for T {
    fn all_columns() -> HashMap<&'static str, Vec<Column>> {
        let mut columns = HashMap::new();
        columns.insert(
            Self::table_name(),
            Self::columns()
                .into_iter()
                .filter(|c| !c.foreign())
                .collect(),
        );
        for reference in Self::references() {
            columns.extend(reference.all_columns);
        }
        columns
    }

    fn select_all() -> Select {
        let mut done = Vec::new();
        let mut todo = Self::references();
        let mut results = Vec::new();

        while !todo.is_empty() {
            let reference = todo.first().expect("Just checked").clone();
            results.push(format!(
                "{} JOIN {} ON {}",
                if ((reference.join as u8)
                    & ((ReferenceJoin::ExaclyOne as u8) | (ReferenceJoin::Multiple as u8)))
                    == 0
                {
                    "LEFT"
                } else {
                    "INNER"
                },
                reference.to_table_name,
                reference
                    .identifiers
                    .iter()
                    .map(|column| {
                        let (c1, c2) = column
                            .split_once(';')
                            .expect("Reference identifiers is in the format col1;col2");
                        format!(
                            "{}.{} = {}.{}",
                            reference.to_table_name,
                            c2,
                            reference.from_table_name,
                            c1,
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(" AND ")
            ));
            for ref2 in &reference.references {
                if done.contains(ref2) || todo.contains(ref2) {
                    continue;
                }
                todo.push(ref2.clone());
            }
            done.push(todo.remove(0));
        }
        results.insert(0, Self::table_name().to_owned());

        Select::new(Self::all_columns(), results)
    }
}
