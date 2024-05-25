use mysql::{FromRowError, Row};
pub use rust_query_macro_mysql::MySQLRow;

pub trait MySQLRow {
    #[allow(clippy::missing_errors_doc)]
    fn from_row_ref(row: &Row) -> Result<Self, FromRowError>
    where
        Self: Sized;
}

pub trait RowFlatten<T> {
    fn row_flatten(&self) -> Vec<T>;
}
