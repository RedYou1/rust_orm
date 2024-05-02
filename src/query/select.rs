use std::collections::HashMap;

use crate::table::Column;

pub struct Select {
    columns: HashMap<&'static str, Vec<Column>>,
    tables_join: Vec<String>,
}

impl Select {
    pub fn new(columns: HashMap<&'static str, Vec<Column>>, tables_join: Vec<String>) -> Self {
        Self {
            columns,
            tables_join,
        }
    }
}

impl ToString for Select {
    fn to_string(&self) -> String {
        format!(
            "SELECT {} FROM {};",
            self.columns
                .iter()
                .flat_map(|(key, values)| values
                    .iter()
                    .map(|value| format!("{}.{}", key, value.name()))
                    .collect::<Vec<String>>())
                .collect::<Vec<String>>()
                .join(", "),
            self.tables_join.join(" ")
        )
    }
}
