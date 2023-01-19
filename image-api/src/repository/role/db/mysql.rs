use std::result::Result;
use mysql::*;
use mysql::prelude::*;

use crate::{
    db::{ DBError, DBImpl, get_db_connection },
    model::{ role::Role, user_permissions::UserPermissions },
    repository::role::RoleRepository,
};

pub struct MySQLRoleRepository {}

impl RoleRepository for MySQLRoleRepository {
    fn get_all(&self) -> Result<Vec<Role>, DBError> {
        let mut conn: PooledConn = get_db_connection();
        let statement = conn.prep(r"SELECT
            ID, ROLE_NAME, CREATE_IMAGE, READ_IMAGE, MODIFY_IMAGE,
            DELETE_IMAGE, READ_RENDITIONS, CREATE_RENDITIONS, MODIFY_RENDITIONS,
            DELETE_RENDITIONS, READ_PROJECT, CREATE_PROJECT, MODIFY_PROJECT,
            DELETE_PROJECT, READ_USER, CREATE_USER, MODIFY_USER, DELETE_USER,
            PUBLISH, PUBLISH_ALL, ACCESS_ALL_PROJECTS
            FROM USER_ROLE
        ").unwrap();

        let rows_wrapped: mysql::Result<Vec<Row>> =
            conn.exec(statement, Params::Empty);

        match rows_wrapped {
            Ok (rows) => {
                let mut roles: Vec<Role> = vec![];

                for row_ref in rows.iter() {
                    let mut row: Row = row_ref.clone();

                    let permissions = UserPermissions {
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
                        access_all_projects: row.take("ACCESS_ALL_PROJECTS").unwrap()
                    };

                    roles.push(Role {
                        id: row.take("ID").unwrap(),
                        role_name: row.take("ROLE_NAME").unwrap(),
                        permissions
                    });

                }

                Ok (roles)
            }

            Err (e) => {
                eprintln!("Error while getting rendition from query: {}", e);

                Err(DBError::OtherError)
            }
        }
    }

    //fn get(&self, id: u16) -> Result<Role, DBError> {}

    fn add(&self, role: Role) {
        let error_msg: String = String::from("Error Inserting Data!");

        let mut conn = get_db_connection();
        //let transaction_result = conn.start_transaction(TxOpts::default());

//        match transaction_result {
//            Ok (mut tx) => {
                //let res = tx.exec_drop(
                conn.exec_drop(
                    r"INSERT INTO USER_ROLE (
                        ROLE_NAME, CREATE_IMAGE, READ_IMAGE, MODIFY_IMAGE,
                        DELETE_IMAGE, READ_RENDITIONS, CREATE_RENDITIONS,
                        MODIFY_RENDITIONS, DELETE_RENDITIONS, READ_PROJECT,
                        CREATE_PROJECT, MODIFY_PROJECT, DELETE_PROJECT,
                        READ_USER, CREATE_USER, MODIFY_USER, DELETE_USER,
                        PUBLISH, PUBLISH_ALL, ACCESS_ALL_PROJECTS
                    ) VALUES (
                        :role_name, :create_image, :read_image, :modify_image,
                        :delete_image, :read_renditions, :create_renditions,
                        :modify_renditions, :delete_renditions, :read_project,
                        :create_project, :modify_project, :delete_project,
                        :read_user, :create_user, :modify_user, :delete_user,
                        :publish, :publish_all, :access_all_projects
                    )",
                    params! {
                        "role_name" => &role.role_name,
                        "create_image" => &role.permissions.create_image,
                        "read_image" => &role.permissions.read_image,
                        "modify_image" => &role.permissions.modify_image,
                        "delete_image" => &role.permissions.delete_image,
                        "read_renditions" => &role.permissions.read_renditions,
                        "create_renditions" => &role.permissions.create_renditions,
                        "modify_renditions" => &role.permissions.modify_renditions,
                        "delete_renditions" => &role.permissions.delete_renditions,
                        "read_project" => &role.permissions.read_project,
                        "create_project" => &role.permissions.create_project,
                        "modify_project" => &role.permissions.modify_project,
                        "delete_project" => &role.permissions.delete_project,
                        "read_user" => &role.permissions.read_user,
                        "create_user" => &role.permissions.create_user,
                        "modify_user" => &role.permissions.modify_user,
                        "delete_user" => &role.permissions.delete_user,
                        "publish" => &role.permissions.publish,
                        "publish_all" => &role.permissions.publish_all,
                        "access_all_project" => &role.permissions.access_all_projects,
                    }
                );

//                match res {
//                    Ok (_) => {
//                        println!("Data Inserted!");
//
//                        let row_wrapped: Result<Option<Row>, Error> = tx.exec_first(
//                            r"SELECT LAST_INSERT_ID() as LID;",
//                            Params::Empty,
//                        );
//
//                        match row_wrapped {
//                            Ok(row_option) => {
//                                match row_option {
//                                    Some (mut row) => {
//                                        match row.take("LID") {
//                                            Some (id) => {
//                                                let c_res = tx.commit();
//                                                
//                                                match c_res {
//                                                    Ok (_) => Ok(id),
//                                                    Err (_) => Err(error_msg)
//                                                }
//                                            }
//
//                                            None => {
//                                                let c_res = tx.rollback();
//                                                
//                                                match c_res {
//                                                    Ok (_) => Err(error_msg),
//                                                    Err (_) => Err(error_msg)
//                                                }
//                                            }
//                                        }
//                                    }
//
//                                    None => {
//                                        let c_res = tx.rollback();
//                                        
//                                        match c_res {
//                                            Ok (_) => Err(error_msg),
//                                            Err (_) => Err(error_msg)
//                                        }
//                                    }
//                                }
//                            }
//
//                            Err(_e) => {
//                                let c_res = tx.rollback();
//                                
//                                match c_res {
//                                    Ok (_) => Err(error_msg),
//                                    Err (_) => Err(error_msg)
//                                }
//                            }
//                        }
//                    }
//
//                    Err (_) => {
//                        let c_res = tx.rollback();
//                        
//                        match c_res {
//                            Ok (_) => Err(error_msg),
//                            Err (_) => Err(error_msg)
//                        }
//                    }
//                }
//            }
//
//            Err (_e) => Err(String::from("Error initializing transaction"))
//        }
    }

    fn update(&self, role: Role) -> Result<(), String> {
        let mut conn = get_db_connection();

        match conn.exec_drop(r"UPDATE USER_ROLE SET
            ROLE_NAME = :role_name, CREATE_IMAGE = :create_image,
            READ_IMAGE = :read_image, MODIFY_IMAGE = :modify_image,
            DELETE_IMAGE = :delete_image, READ_RENDITIONS = :read_renditions,
            CREATE_RENDITIONS = :create_renditions,
            MODIFY_RENDITIONS = :modify_renditions,
            DELETE_RENDITIONS = :delete_renditions,
            READ_PROJECT = :read_project, CREATE_PROJECT = :create_project,
            MODIFY_PROJECT = :modify_project, DELETE_PROJECT = :delete_project,
            READ_USER = :read_user, CREATE_USER = :create_user,
            MODIFY_USER = :modify_user, DELETE_USER = :delete_user,
            PUBLISH = :publish, PUBLISH_ALL = :publish_all,
            ACCESS_ALL_PROJECTS = :ACCESS_ALL_PROJECTS
            ",
            params! {
                "role_name" => &role.role_name,
                "create_image" => &role.permissions.create_image,
                "read_image" => &role.permissions.read_image,
                "modify_image" => &role.permissions.modify_image,
                "delete_image" => &role.permissions.delete_image,
                "read_renditions" => &role.permissions.read_renditions,
                "create_renditions" => &role.permissions.create_renditions,
                "modify_renditions" => &role.permissions.modify_renditions,
                "delete_renditions" => &role.permissions.delete_renditions,
                "read_project" => &role.permissions.read_project,
                "create_project" => &role.permissions.create_project,
                "modify_project" => &role.permissions.modify_project,
                "delete_project" => &role.permissions.delete_project,
                "read_user" => &role.permissions.read_user,
                "create_user" => &role.permissions.create_user,
                "modify_user" => &role.permissions.modify_user,
                "delete_user" => &role.permissions.delete_user,
                "publish" => &role.permissions.publish,
                "publish_all" => &role.permissions.publish_all,
                "access_all_project" => &role.permissions.access_all_projects,
            }
        ) {
            Ok(_) => Ok(()),

            Err (e) => {
                eprintln!("{}", e);

                return Err(String::from("Unable to update role."));
            }
        }
    }

    /**
     * TODO: Implement this!
     */
    fn remove(&self, id: Role) {
        todo!("Implement this!");
    }

    /**
     * TODO: Implement this!
     */
    fn remove_item(&self, id: u32) {
        todo!("Implement this!");
    }
}

