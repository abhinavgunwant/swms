/**
 * Tells us how to connect to a data source.
 */
pub trait DBContext {
    /**
     * Connection string
     */
    fn connection_string(&self) -> String;

    /**
     * Return the name of the DB.
     */
    fn db_name(&self) -> String;

    /**
     * Do a basic check of database tables and system.
     */
    fn systems_check(&self);
}

pub struct MySQLContext {
    db_name: String,
    connection_string: String
}

impl DBContext for MySQLContext {
    fn db_name(&self) -> String {
        return self.db_name;
    }
    
    fn connection_string(&self) -> String {
        return self.connection_string;
    }

    fn systems_check(&self) {
        println!("System check starting...");
    }
}

impl<T: ?Sized> DBContext for Box<T> where T: DBContext {
    fn db_name(&self) -> String {
        (**self).db_name()
    }

    fn connection_string(&self) -> String {
        (**self).connection_string()
    }

    fn systems_check(&self) {
        (**self).systems_check();
    }
}
