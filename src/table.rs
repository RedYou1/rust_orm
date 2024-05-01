pub use crate::column::Column;
use crate::query::select::Select;

pub trait Table {
    fn table_name() -> &'static str;
    fn columns() -> Vec<Column>;
}

pub trait Queries {
    fn select_all() -> Select;
}

impl<T: Table> Queries for T {
    fn select_all() -> Select {
        Select::new(Self::table_name(), Self::columns())
    }
}
