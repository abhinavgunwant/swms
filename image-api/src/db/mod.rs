use dbcontext::{ DBContext, MySQLContext };

mod dbcontext;

pub enum DBImpl {
    MYSQL = 1
}

/**
 * Main objective of this function is to read the config and return appropriate
 * db context object.
 */
fn get_db_context(db: DBImpl) -> impl DBContext {
    match db {
        DBImpl::MYSQL => MySQLContext {
            connection_string: "mysql://root:Welcome1@localhost:3306/test".to_string(),
            db_name: "mysql".to_string()
        }
    }
}
