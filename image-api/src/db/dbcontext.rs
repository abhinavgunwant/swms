use crate::db::DBImpl;
use std::hash::{ Hash, Hasher };

/**
 * Tells us how to connect to a data source.
 */
//pub trait DBContext: Sized + Clone {
//    /**
//     * Connection string
//     */
//    fn connection_string(&self) -> String;
//
//    /**
//     * Return the name of the DB.
//     */
//    fn db_name(&self) -> String;
//
//    /**
//     * Do a basic check of database tables and system.
//     */
//    fn systems_check(&self);
//}

#[derive(Clone)]
pub struct DBContext {
    pub dbimpl: DBImpl,
    pub connection_string: String,
    pub db_name: String
}

pub struct MySQLContext {
    pub db_name: String,
    pub connection_string: String
}

//impl DBContext for MySQLContext {
//    fn db_name(&self) -> String {
//        self.db_name.clone()
//    }
//    
//    fn connection_string(&self) -> String {
//        self.connection_string.clone()
//    }
//
//    fn systems_check(&self) {
//        println!("Performing system check for MySQL Database");
//
//        println!("Creating user_role table...");
//        // TODO: Create user_role table if it doesn't exist.
//
//        println!("Creating user table...");
//        // TODO: Create user table if it doesn't exist.
//
//        println!("Creating project table...");
//        // TODO: Create project table if it doesn't exist.
//        
//        println!("Creating folder table...");
//        // TODO: Create folder table if it doesn't exist.
//
//        println!("Creating image table...");
//        // TODO: Create image table if it doesn't exist.
//
//        println!("Creating image_rendition table...");
//        // TODO: Create image_rendition table if it doesn't exist.
//    }
//}
//
//impl<T: ?Sized> DBContext for Box<T> where T: DBContext {
//    fn db_name(&self) -> String {
//        (**self).db_name()
//    }
//
//    fn connection_string(&self) -> String {
//        (**self).connection_string()
//    }
//
//    fn systems_check(&self) {
//        (**self).systems_check();
//    }
//}
//
//// for cached
//impl Clone for MySQLContext {
//    fn clone(&self) -> Self {
//        *self
//    }
//
//    fn clone_from(&mut self, source: &Self) {
//        *self = *source
//    }
//}

//impl Copy for DBContext {}

impl DBContext {
    pub fn new(di: DBImpl, cs: String, dn: String) -> Self {
        Self {
            dbimpl: di, connection_string: cs, db_name: dn
        }
    }

    pub fn connection_string(&self) -> String {
        self.connection_string.clone()
    }

    pub fn set_connection_string(&mut self, con_str: String) {
        self.connection_string = con_str;
    }

    pub fn db_name(&self) -> String {
        self.db_name.clone()
    }

    pub fn set_db_name(&mut self, dbname: String) {
        self.db_name = dbname;
    }
}

//impl Clone for DBContext {
//    fn clone(&self) -> Self {
//        *self
//    }
//
//    fn clone_from(&mut self, source: &Self) {
//        *self = *source
//    }
//}

impl PartialEq<Self> for DBContext {
    fn eq(&self, other: &Self) -> bool {
        self.connection_string.eq(&other.connection_string)
            && self.db_name.eq(&other.db_name)
            && self.dbimpl == other.dbimpl
    }
}

impl Hash for DBContext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dbimpl;
    }
}

