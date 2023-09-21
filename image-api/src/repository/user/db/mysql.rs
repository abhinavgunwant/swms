use std::result::Result;
use chrono::Utc;
use mysql::*;
use mysql::prelude::*;
use crate::model::{
    user_permissions::UserPermissions, user_search::UserSearch
};
use crate::repository::user::{ User, UserRepository };
use crate::db::{
    utils::mysql::{ get_row_from_query, get_rows_from_query },
    DBError, get_db_connection
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

fn get_users_from_rows(row_wrapped: Result<Vec<Row>, Error>)
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
            params! { "id" => id },
        ))
    }

    fn get_from_login_id(&self, login_id: String) -> std::result::Result<User, DBError> {
        println!("-> Getting projects for: {}", login_id);

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

    fn get_permissions(&self, login_id: String) -> Result<UserPermissions, String> {
        let row_result = get_row_from_query(
            r"SELECT
                R.CREATE_IMAGE, R.READ_IMAGE, R.MODIFY_IMAGE, R.DELETE_IMAGE,
                R.READ_RENDITIONS, R.CREATE_RENDITIONS, R.MODIFY_RENDITIONS,
                R.DELETE_RENDITIONS, R.READ_PROJECT, R.CREATE_PROJECT,
                R.MODIFY_PROJECT, R.DELETE_PROJECT, R.READ_USER, R.CREATE_USER,
                R.MODIFY_USER, R.DELETE_USER, R.PUBLISH, R.PUBLISH_ALL,
                R.ACCESS_ALL_PROJECTS
            FROM USER_ROLE R, USER U
            WHERE U.LOGIN_ID = :login_id AND U.USER_ROLE = R.ID",
            params! { "login_id" => login_id },
        );

        match row_result {
            Ok (row_option) => {
                match row_option {
                    Some (row_ref) => {
                        let mut row = row_ref.clone();

                        Ok(UserPermissions {
                            create_image: row.take("CREATE_IMAGE").unwrap(),
                            read_image: row.take("READ_IMAGE").unwrap(),
                            modify_image: row.take("MODIFY_IMAGE").unwrap(),
                            delete_image: row.take("DELETE_IMAGE").unwrap(),
                            read_renditions: row.take("READ_RENDITIONS").unwrap(),
                            create_renditions: row.take("CREATE_RENDITIONS").unwrap(),
                            modify_renditions: row.take("MODIFY_RENDITIONS").unwrap(),
                            delete_renditions: row.take("DELETE_RENDITIONS").unwrap(),
                            read_project: row.take("READ_PROJECT").unwrap(),
                            create_project: row.take("CREATE_PROJECT").unwrap(),
                            modify_project: row.take("MODIFY_PROJECT").unwrap(),
                            delete_project: row.take("DELETE_PROJECT").unwrap(),
                            read_user: row.take("READ_USER").unwrap(),
                            create_user: row.take("CREATE_USER").unwrap(),
                            modify_user: row.take("MODIFY_USER").unwrap(),
                            delete_user: row.take("DELETE_USER").unwrap(),
                            publish: row.take("PUBLISH").unwrap(),
                            publish_all: row.take("PUBLISH_ALL").unwrap(),
                            access_all_projects: row.take("ACCESS_ALL_PROJECTS").unwrap(),
                        })
                    }

                    None => {
                        Err(String::from(
                            "User permissions not found, does the user exist?"
                        ))
                    }
                }
            }

            Err (_e) => {
                Err(String::from(
                    "There was some error retrieving user permissions"
                ))
            }
        }
    }

    fn get_all(&self) -> Result<Vec<User>, DBError> {
        get_users_from_rows(get_rows_from_query(
            r"SELECT
                ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
                CREATED_ON, MODIFIED_ON, NAME, PASSWORD
            FROM USER",
            Params::Empty,
        ))
    }

    fn get_all_paged(&self, page: u32, page_length: u32) -> Result<Vec<User>, DBError> {
        get_users_from_rows(get_rows_from_query(
            r"SELECT
                ID, LOGIN_ID, EMAIL, USER_ROLE, LAST_LOGIN_ON, CREATED_BY, MODIFIED_BY,
                CREATED_ON, MODIFIED_ON, NAME
            FROM USER LIMIT :page1, :page2",
            params! { "page1" => page*page_length, "page2" => page }
        ))
    }

    fn search_from_name(&self, name_query: String, page_length: u32)
        -> Result<Vec<UserSearch>, DBError> {
        let nq: String = format!("%{}%", name_query.to_uppercase());
        let rows_result = get_rows_from_query(
            r"SELECT ID, NAME FROM USER
            WHERE UPPER(NAME) LIKE :nq LIMIT :page_length",
            params! { "page_length" => page_length, "nq" => nq }
        );

        match rows_result  {
            Ok (rows) => {
                let mut users: Vec<UserSearch> = vec![];

                for row_ref in rows.iter() {
                    let mut row = row_ref.clone();
                    let mut add: bool = true;
                    let mut id: u32 = 0;
                    let mut name: String = String::from("");

                    match row.take_opt("ID") {
                        Some(id_res) => {
                            match id_res {
                                Ok (user_search_id) => {
                                    id = user_search_id;
                                }

                                Err (_e) => {
                                    add = false;
                                }
                            }
                        }

                        None => {
                            add = false;
                        }
                    }

                    if add {
                        match row.take_opt("NAME") {
                            Some(name_res) => {
                                match name_res {
                                    Ok (user_search_name) => {
                                        name = user_search_name;
                                    }

                                    Err (_e) => {
                                        add = false;
                                    }
                                }
                            }

                            None => {
                                add = false;
                            }
                        }
                    }

                    if add {
                        users.push(UserSearch { id, name });
                    }
                }

                Ok (users)
            }

            Err (e) => {
                eprintln!("Error while getting rendition from query: {}", e);

                Err(DBError::OtherError)
            }
        }
    }

    fn add(&self, user: User) -> Result<u32, String> {
        let error_msg: String = String::from("Error Inserting Data!");

        let mut conn = get_db_connection();
        let transaction_result = conn.start_transaction(TxOpts::default());

        match transaction_result {
            Ok (mut tx) => {
                let res = tx.exec_drop(
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
                );

                match res {
                    Ok (_) => {
                        println!("Data Inserted!");

                        let row_wrapped: Result<Option<Row>, Error> = tx.exec_first(
                            r"SELECT LAST_INSERT_ID() as LID;",
                            Params::Empty,
                        );

                        match row_wrapped {
                            Ok(row_option) => {
                                match row_option {
                                    Some (mut row) => {
                                        match row.take("LID") {
                                            Some (id) => {
                                                let c_res = tx.commit();
                                                
                                                match c_res {
                                                    Ok (_) => Ok(id),
                                                    Err (_) => Err(error_msg)
                                                }
                                            }

                                            None => {
                                                let c_res = tx.rollback();
                                                
                                                match c_res {
                                                    Ok (_) => Err(error_msg),
                                                    Err (_) => Err(error_msg)
                                                }
                                            }
                                        }
                                    }

                                    None => {
                                        let c_res = tx.rollback();
                                        
                                        match c_res {
                                            Ok (_) => Err(error_msg),
                                            Err (_) => Err(error_msg)
                                        }
                                    }
                                }
                            }

                            Err(_e) => {
                                let c_res = tx.rollback();
                                
                                match c_res {
                                    Ok (_) => Err(error_msg),
                                    Err (_) => Err(error_msg)
                                }
                            }
                        }
                    }

                    Err (e) => {
                        eprintln!("Error inserting data: {}", e);
                        eprintln!("Rolling back the transaction!");

                        let c_res = tx.rollback();
                        
                        match c_res {
                            Ok (_) => Err(error_msg),
                            Err (_) => Err(error_msg)
                        }
                    }
                }
            }

            Err (_e) => Err(String::from("Error initializing transaction"))
        }
    }

    fn update(&self, user: User) -> Result<(), String> {
        let mut conn = get_db_connection();

        match conn.exec_drop(
            r"UPDATE USER SET NAME = :name, EMAIL = :email,
            USER_ROLE = :user_role WHERE ID = :id",
            params! {
                "id" => user.id,
                "name" => user.name,
                "email" => user.email,
                "user_role" => user.user_role,
            }
        ) {
            Ok(_) => Ok(()),

            Err (e) => {
                eprintln!("Error updating user: {}", e);

                return Err(String::from("Unable to update user."));
            }
        }
    }

    fn remove(&self, _id: User) {
        // TODO: Implement
        println!("Removing an user");
    }

    fn remove_item(&self, _id: u32) {
        // TODO: Implement
        println!("Removing an user item");
    }
}
