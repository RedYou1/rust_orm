use std::collections::HashMap;

use crate::table::Column;

pub struct Select {
    columns: HashMap<&'static str, Vec<Column>>,
    tables_join: Vec<String>,
    wheres: Option<String>,
    orderbys: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl Select {
    #[must_use]
    pub fn new(columns: HashMap<&'static str, Vec<Column>>, tables_join: Vec<String>) -> Self {
        Self {
            columns,
            tables_join,
            wheres: None,
            orderbys: None,
            limit: None,
            offset: None,
        }
    }
}

impl ToString for Select {
    fn to_string(&self) -> String {
        format!(
            "SELECT {} FROM {}{}{}{}{};",
            self.columns
                .iter()
                .flat_map(|(key, values)| values
                    .iter()
                    .map(|value| format!("{}.{} AS {}_{}", key, value.name(), key, value.name()))
                    .collect::<Vec<String>>())
                .collect::<Vec<String>>()
                .join(", "),
            self.tables_join.join(" "),
            self.wheres
                .as_ref()
                .map_or(String::new(), |s| format!(" WHERE {s}")),
            self.orderbys
                .as_ref()
                .map_or(String::new(), |s| format!(" ORDER BY {s}")),
            self.limit
                .as_ref()
                .map_or(String::new(), |s| format!(" LIMIT {s}")),
            self.offset
                .as_ref()
                .map_or(String::new(), |s| format!(" OFFSET {s}"))
        )
    }
}

impl Select {
    pub fn wheres(&mut self, wheres: String) -> &mut Self {
        self.wheres = Some(wheres);
        self
    }
    pub fn orderbys(&mut self, orderbys: String) -> &mut Self {
        self.orderbys = Some(orderbys);
        self
    }
    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}
