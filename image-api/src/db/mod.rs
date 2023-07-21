pub mod dbcontext;
pub mod utils;

use std::fmt::{ Display, Formatter, Result as FmtResult };
use mysql::{ PooledConn, Pool };
use dbcontext::{ DBContext, MySQLContext };
use cached::proc_macro::cached;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CURRENT_DB: DBContext = get_db_context();
}

#[derive(PartialEq)]
pub enum DBError {
    NOT_FOUND,
    OtherError
}

impl Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NOT_FOUND => {
                return write!(f, "DB: Not Found");
            }

            Self::OtherError => {
                return write!(f, "DB: Some other error occured");
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy)]
pub enum DBImpl {
    MYSQL = 1// ,
    // Implementation pending for below DBs:
    // MONGODB = 2,
    // ORACLE = 3,
}

impl Clone for DBImpl {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

/**
 * Reads config and returns appropriate db context object ().
 */
#[cached]
pub fn get_db_context() -> DBContext {
    DBContext::new(
        DBImpl::MYSQL,
        "mysql://root:Welcome1@localhost:3306/dam".to_string(),
        "mysql".to_string()
    )
}

pub fn get_db_connection() -> PooledConn {
    let dbc:DBContext = get_db_context();
    let pool = Pool::new(String::as_str(&dbc.connection_string));
    let conn: PooledConn = pool.unwrap().get_conn().unwrap();

    conn
}

