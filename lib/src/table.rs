use std::collections::HashMap;

pub use crate::column::Column;
use crate::query::select::Select;
pub use rust_query_macro::Table;

#[derive(Debug, Clone, Copy)]
pub enum ReferenceJoin {
    OneOrNone = 1,
    ExaclyOne = 2,
    Multiple = 4,
    MultipleOption = 8,
    MultipleEmpty = 16,
    MultipleOptionEmpty = 32,
}
impl From<u8> for ReferenceJoin{
    fn from(value: u8) -> Self {
        match value {
            1 => ReferenceJoin::OneOrNone,
            2 => ReferenceJoin::ExaclyOne,
            4 => ReferenceJoin::Multiple,
            8 => ReferenceJoin::MultipleOption,
            16 => ReferenceJoin::MultipleEmpty,
            32 => ReferenceJoin::MultipleOptionEmpty,
            _ => panic!("ReferenceJoin from u8 not found")
        }
    }
}


pub struct Reference {
    pub join: ReferenceJoin,
    pub column_prefix: &'static str,
    pub table_name: &'static str,
    pub identifiers: Vec<Column>,
    pub other_columns: Vec<Column>,
}

pub trait Table {
    fn identifiers() -> Vec<Column>;
    fn table_name() -> &'static str;
    fn columns() -> Vec<Column>;
    fn references() -> Vec<Reference>;
}

pub trait Queries {
    fn select_all() -> Select;
}

impl<T: Table> Queries for T {
    fn select_all() -> Select {
        let others = T::references();

        let mut joins: Vec<String> = others
            .iter()
            .map(|reference| {
                format!(
                    "{} JOIN {} ON {}",
                    if ((reference.join as u8) & ((ReferenceJoin::ExaclyOne as u8) | (ReferenceJoin::Multiple as u8))) == 0 {
                        "LEFT"
                    } else {
                        "INNER"
                    },
                    reference.table_name,
                    reference
                        .identifiers
                        .iter()
                        .map(|column| {
                            format!(
                                "{}.{} = {}.{}_{}",
                                reference.table_name,
                                column.name(),
                                T::table_name(),
                                reference.column_prefix,
                                column.name()
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(" AND ")
                )
            })
            .collect();
        joins.insert(0, T::table_name().to_owned());

        let mut columns = HashMap::with_capacity(others.len());
        columns.insert(
            T::table_name(),
            T::columns().into_iter().filter(|c| !c.foreign()).collect(),
        );
        for reference in others {
            columns.insert(
                reference.table_name,
                reference
                    .other_columns
                    .into_iter()
                    .filter(|c| !c.foreign())
                    .collect(),
            );
        }

        Select::new(columns, joins)
    }
}
