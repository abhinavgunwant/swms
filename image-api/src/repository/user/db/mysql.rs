use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use crate::repository::user::{ User, UserRepository };
use crate::db::{ DBError, get_db_context, dbcontext::DBContext };
use crate::authtools::generate_password_hash;

const SELECT_ONE: &'static str = r"
    SELECT
        ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
        CREATED_ON, MODIFIED_ON, NAME, PASSWORD
    FROM USER WHERE ID = :id";

const SELECT_ONE_LOGIN_ID: &'static str = r"
    SELECT
        ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
        CREATED_ON, MODIFIED_ON, NAME, PASSWORD
    FROM USER WHERE LOGIN_ID = :login_id";

const SELECT_ALL: &'static str = r"
    SELECT
        ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
        CREATED_ON, MODIFIED_ON, NAME, PASSWORD
    FROM USER";

const ADD_ONE: &'static str = r"
    INSERT INTO USER (
        LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
        CREATED_ON, MODIFIED_ON, NAME, PASSWORD
    ) VALUES (
        :login_id, :email, :user_role, NULL, :created_by, :modified_by,
        CURRENT_TIMESTAMP(), CURRENT_TIMESTAMP(), :name, :password
    )
";

fn get_user_from_row(row_wrapped: Option<&Row>)
    -> std::result::Result<User, DBError> {

    match row_wrapped {
        Some(row_ref) => {
            let mut row = row_ref.clone();

            Ok(User {
                id: row.take("ID").unwrap(),
                name: row.take("NAME").unwrap(),
                password: String::from("HIDDEN"),
                login_id: row.take("LOGIN_ID").unwrap(),
                email: row.take("EMAIL").unwrap(),
                user_role: row.take("USER_ROLE").unwrap(),
                created_by: row.take("CREATED_BY").unwrap(),
                modified_by: row.take("MODIFIED_BY").unwrap(),
                created_on: Utc::now(),
                modified_on: Utc::now(),
                last_login_on: Utc::now(),
            })
        }

        None => {
            Err(DBError::NOT_FOUND)
        }
    }
}

pub struct MySQLUserRepository {}

impl UserRepository for MySQLUserRepository {
    fn get(&self, id: u32) -> std::result::Result<User, DBError> {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();
        let statement = conn.prep(SELECT_ONE).unwrap();

        let rows: Vec<Row> = conn.exec(statement, params! {"id" => id}).unwrap();

        get_user_from_row(rows.get(0))
    }

    fn get_from_login_id(&self, login_id: String) -> std::result::Result<User, DBError> {
        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        let statement = conn.prep(SELECT_ONE_LOGIN_ID).unwrap();

        let rows: Vec<Row> = conn.exec(
            &statement,
            params! { "login_id" => login_id}
        ).unwrap();

        get_user_from_row(rows.get(0))
    }

    fn get_all(&self) -> Vec::<User> {
        let user_result = self.get(0);

        let mut users = Vec::new();

        match user_result {
            Ok(user) => {
                users.push(user);
            }

            Err(e) => {
                println!("Error while getting all users!!!!");
            }
        }

        users
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Vec::<User> {
        self.get_all()
    }

    fn add(&self, user: User) {
        println!("adding an user");

        let dbc:DBContext = get_db_context();

        let pool = Pool::new(String::as_str(&dbc.connection_string));

        let mut conn = pool.unwrap().get_conn().unwrap();

        conn.exec_drop(ADD_ONE, params! {
            "name" => &user.name,
            "login_id" => &user.login_id,
            "password" => generate_password_hash(user.password),
            "email" => &user.email,
            "user_role" => &user.user_role,
            "created_by" => &user.created_by,
            "modified_by" => &user.modified_by,
        }).expect("Error while creating user");
    }

    fn update(&self, user: User) {
        println!("Updating an user");
    }

    fn remove(&self, id: User) {
        println!("Removing an user");
    }

    fn remove_item(&self, id: u32) {
        println!("Removing an user item");
    }
}
