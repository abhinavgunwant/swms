use mysql::*;
use mysql::prelude::*;

use crate::db::{ get_db_connection, DBError };

pub fn get_rows_from_query(query: &str, params: Params) -> Result<Vec<Row>> {
    let mut conn: PooledConn = get_db_connection();
    let statement = conn.prep(query).unwrap();

    conn.exec(statement, params)
}

pub fn get_row_from_query(query: &str, params: Params) -> Result<Option<Row>> {
    let mut conn: PooledConn = get_db_connection();
    let statement = conn.prep(query).unwrap();

    conn.exec_first(statement, params)
}
