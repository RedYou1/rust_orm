use std::env;

use dotenv::dotenv;
use mysql::{Error, Pool};
use once_cell::sync::Lazy;

static mut DATABASE: Lazy<Pool> = Lazy::new(|| {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    Pool::new(database_url.as_str()).expect("DATABASE failed to initialize")
});

pub fn get_conn() -> Result<mysql::PooledConn, Error> {
    unsafe { DATABASE.clone() }.get_conn()
}
