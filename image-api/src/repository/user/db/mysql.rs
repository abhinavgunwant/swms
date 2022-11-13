use std::result::Result;
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use crate::repository::user::{ User, UserRepository };
use crate::db::{
    utils::mysql::{ get_row_from_query, get_rows_from_query },
    DBError, get_db_context, dbcontext::DBContext, get_db_connection
};
use crate::auth::pwd_hash::generate_password_hash;

fn get_user_from_row(row_wrapped: Result<Option<Row>, Error>)
    -> Result<User, DBError> {

    match row_wrapped {
        Ok (row_option) => {
            match row_option {
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

                None => Err(DBError::NOT_FOUND)
            }
        }

        Err (e) => {
            eprintln!("Error while getting rendition from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

fn get_users_from_row(row_wrapped: Result<Vec<Row>, Error>)
    -> Result<Vec<User>, DBError> {
    match row_wrapped {
        Ok (rows) => {
            let mut users: Vec<User> = vec![];

            for row_ref in rows.iter() {
                let mut row = row_ref.clone();

                users.push(User {
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
                });
            }

            Ok (users)
        }

        Err (e) => {
            eprintln!("Error while getting rendition from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

pub struct MySQLUserRepository {}

impl UserRepository for MySQLUserRepository {
    fn get(&self, id: u32) -> std::result::Result<User, DBError> {
        get_user_from_row(get_row_from_query(
            r"SELECT
                ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY,
                MODIFIED_BY, CREATED_ON, MODIFIED_ON, NAME
            FROM USER WHERE ID = :id",
            Params::Empty,
        ))
    }

    fn get_from_login_id(&self, login_id: String) -> std::result::Result<User, DBError> {
        get_user_from_row(get_row_from_query(
            r"SELECT
                ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
                CREATED_ON, MODIFIED_ON, NAME
            FROM USER WHERE LOGIN_ID = :login_id",
            params! { "login_id" => login_id}
        ))
    }

    fn get_password_for_login_id(&self, login_id: String)
        -> std::result::Result<String, DBError> {
        let mut conn = get_db_connection();

        let statement = conn.prep(
            r"SELECT PASSWORD FROM USER WHERE LOGIN_ID = :login_id"
        ).unwrap();

        let rows: Vec<Row> = conn.exec(
            &statement,
            params! { "login_id" => login_id}
        ).unwrap();

        if rows.len() == 0 {
            return Err(DBError::NOT_FOUND);
        }

        match rows.get(0) {
            Some (row_ref) => {
                let mut row = row_ref.clone();
                let password: Option<String> = row.take("PASSWORD");

                match password {
                    Some (password) => Ok (password),
                    None => Err(DBError::NOT_FOUND),
                }
            }

            None => Err(DBError::NOT_FOUND),
        }
    }

    fn get_all(&self) -> Result<Vec<User>, DBError> {
        get_users_from_row(get_rows_from_query(
            r"SELECT
                ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
                CREATED_ON, MODIFIED_ON, NAME, PASSWORD
            FROM USER",
            Params::Empty,
        ))
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<User>, DBError> {
        get_users_from_row(get_rows_from_query(
            r"SELECT
                ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
                CREATED_ON, MODIFIED_ON, NAME
            FROM USER LIMIT :page1, :page2",
            params! { "page1" => page*page_length, "page2" => page }
        ))
    }

    fn add(&self, user: User) {
        let mut conn = get_db_connection();

        conn.exec_drop(
            r"INSERT INTO USER (
                LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
                CREATED_ON, MODIFIED_ON, NAME, PASSWORD
            ) VALUES (
                :login_id, :email, :user_role, NULL, :created_by, :modified_by,
                CURRENT_TIMESTAMP(), CURRENT_TIMESTAMP(), :name, :password
            )",
            params! {
                "name" => &user.name,
                "login_id" => &user.login_id,
                "password" => generate_password_hash(user.password),
                "email" => &user.email,
                "user_role" => &user.user_role,
                "created_by" => &user.created_by,
                "modified_by" => &user.modified_by,
            }
        ).expect("Error while creating user");
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
