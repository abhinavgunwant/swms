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

pub fn process_id_from_row_result (row_result: std::result::Result<Option<Row>, Error>)
    -> std::result::Result<Option<u32>, DBError> {
    match row_result {
        Ok (row_option) => {
            match row_option {
                Some (r) => {
                    let mut row = r;

                    match row.take("ID") {
                        Some (id) => Ok(Some(id)),
                        None => Ok(None),
                    }
                }

                None => Ok(None),
            }
        }

        Err (e) => {
            eprintln!("{}", e);

            Err (DBError::OtherError)
        }
    }
}

