use crate::db::DBImpl;
use std::hash::{ Hash, Hasher };

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
