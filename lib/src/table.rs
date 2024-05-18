use std::collections::HashMap;

pub use crate::column::Column;
use crate::query::select::Select;
pub use rust_query_macro::Table;

pub trait Table {
    fn identifiers() -> Vec<Column>;
    fn table_name() -> &'static str;
    fn columns() -> Vec<Column>;
    fn references() -> Vec<(&'static str, &'static str, Vec<Column>, Vec<Column>)>;
}

pub trait Queries {
    fn select_all() -> Select;
}

impl<T: Table> Queries for T {
    fn select_all() -> Select {
        let others = T::references();

        let mut joins: Vec<String> = others
            .iter()
            .map(|(column_prefix, table_name, identifiers, _)| {
                format!(
                    "INNER JOIN {} ON {}",
                    table_name,
                    identifiers
                        .iter()
                        .map(|column| {
                            format!(
                                "{}.{} = {}.{}_{}",
                                table_name,
                                column.name(),
                                T::table_name(),
                                column_prefix,
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
        for (_, table_name, _, other_columns) in others {
            columns.insert(
                table_name,
                other_columns.into_iter().filter(|c| !c.foreign()).collect(),
            );
        }

        Select::new(columns, joins)
    }
}
