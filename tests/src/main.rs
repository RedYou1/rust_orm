mod connection;
mod test1;

use mysql::Error;

use crate::{connection::get_conn, test1::test1};

#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
pub fn main() -> Result<(), Error> {
    let mut conn = get_conn()?;

    test1(&mut conn);

    println!("Success");
    Ok(())
}
