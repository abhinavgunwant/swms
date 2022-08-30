/**
 * Tells us how to connect to a data source.
 */
trait DBContext {
    /**
     * Connection string
     */
    fn connection_string(&self) -> &str;

    /**
     * Return the name of the DB.
     */
    fn db_name(&self) -> &str;

    /**
     * Do a basic check of database tables and system.
     */
    fn systems_check();
}

struct MySQLContext {
    db_name: str,
    connection_string: str
}

impl DBContext for MySQLContext {
    fn db_name(&self) -> &str {
        &self.db_name;
    }
    
    fn connection_string(&self) -> &str {
        &self.connection_string;
    }

    fn systems_check(&self) {
        println!("System check starting...");
    }
}

/**
 * Main objective of this function is to read the config and return appropriate
 * db context object.
 */
fn get_db_context(name: &str) -> dyn DBContext {
    let mut context: dyn DBContext;
    match name {
        "mysql" => {
            context = MySQLContext {
                connection_string: "mysql://root:Welcome1@localhost:3306/test",
                db_name: "mysql"
            };
        }
    }

    context;
}
