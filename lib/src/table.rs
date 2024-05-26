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

fn parse_reference(
    table_name: &mut String,
    to_table_name: &str,
    prev_reference: Option<&str>,
    reference: &str,
) -> String {
    let prev_table_name = prev_reference
        .and_then(|pr| pr.split_once(" ON "))
        .map_or(table_name.as_ref(), |(table_name, _)| table_name);
    let mut reference: Vec<&str> = reference.split(" ON ").collect();
    if reference.len() == 1 {
        reference.insert(0, to_table_name);
    }
    assert!(
        reference.len() == 2,
        "Wrong format for reference on ForeignKey"
    );
    let [to_table_name, cols] = reference[0..2] else {
        panic!("Never Happening")
    };
    let cols: Vec<&str> = cols.split_whitespace().collect();
    let mut col_table_name = false;
    let cols = cols.chunks_exact(2);
    let c = if cols.remainder()[0].contains('.') {
        cols.remainder()[0].to_owned()
    } else {
        format!("{to_table_name}.{}", cols.remainder()[0])
    };
    let cols: String = cols
        .map(|strs| {
            col_table_name = !col_table_name;
            if strs[0].contains('.') {
                format!("{} {}", strs[0], strs[1],)
            } else {
                format!(
                    "{}.{} {}",
                    if col_table_name {
                        prev_table_name
                    } else {
                        to_table_name
                    },
                    strs[0],
                    strs[1],
                )
            }
        })
        .collect::<Vec<String>>()
        .join(" ");
    *table_name = to_table_name.to_owned();
    format!("{to_table_name} ON {cols} {c}")
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
            let mut table_name = reference.from_table_name.to_owned();
            let ref_type = if ((reference.join as u8)
                & ((ReferenceJoin::ExaclyOne as u8) | (ReferenceJoin::Multiple as u8)))
                == 0
            {
                "LEFT"
            } else {
                "INNER"
            };
            results.push(format!(
                "{ref_type} JOIN {}",
                parse_reference(
                    &mut table_name,
                    reference.to_table_name,
                    None,
                    reference
                        .identifiers
                        .first()
                        .expect("Need at least one referece in the foreign key"),
                )
            ));
            results.extend(
                reference
                    .identifiers
                    .windows(2)
                    .map(|refs| {
                        format!(
                            "{ref_type} JOIN {}",
                            parse_reference(
                                &mut table_name,
                                reference.to_table_name,
                                Some(refs[0].as_str()),
                                refs[1].as_str(),
                            )
                        )
                    })
                    .collect::<Vec<String>>(),
            );
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
