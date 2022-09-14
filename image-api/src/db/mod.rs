use dbcontext::{ DBContext, MySQLContext };
use cached::proc_macro::cached;
use lazy_static::lazy_static;

pub mod dbcontext;

lazy_static! {
    pub static ref CURRENT_DB: DBContext = get_db_context();
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
 * Main objective of this function is to read the config and return appropriate
 * db context object.
 */
//#[cached]
pub fn get_db_context() -> DBContext {
        //if (!CONFIG_READ) {
            //CONFIG_READ = true;
            // match db {
//                DBImpl::MYSQL => DBContext {
//                    dbimpl: db,
//                    connection_string: "mysql://root:Welcome1@localhost:3306/test".to_string(),
//                    db_name: "mysql".to_string()
//                }
                // DBImpl::MYSQL => 
                DBContext::new(
                    DBImpl::MYSQL,
                    "mysql://root:Welcome1@localhost:3306/test".to_string(),
                    "mysql".to_string()
                )
            // }
        //} else {
            // TODO: initialized the context in the first time!
            //return MySQLContext {
                    //connection_string: "mysql://root:Welcome1@localhost:3306/test".to_string(),
                    //db_name: "mysql".to_string()
                //}
        //}
    //}
}

