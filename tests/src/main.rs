mod connection;
mod test1;
mod test2;

use mysql::{prelude::Queryable, Error};

use crate::{connection::get_conn, test1::test1, test2::test2};

#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
pub fn main() -> Result<(), Error> {
    let mut conn = get_conn()?;
    conn.query_drop(std::fs::read_to_string("db.sql")?)?;

    test1(&mut conn);
    test2(&mut conn);

    println!("Success");
    Ok(())
}
