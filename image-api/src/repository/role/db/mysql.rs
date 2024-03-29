use std::result::Result;

use log::{ info, error };
use mysql::*;
use mysql::prelude::*;

use crate::{
    db::utils::mysql::{ get_row_from_query, get_rows_from_query },
    model::{ role::Role, user_permissions::UserPermissions },
    repository::role::RoleRepository, server::db::DBError,
};

pub struct MySQLRoleRepository {
    pub connection: PooledConn,
}

fn get_role_from_row(mut row: Row) -> Role {
    Role {
        id: row.take("ID").unwrap(),
        role_name: row.take("ROLE_NAME").unwrap(),
        permissions: UserPermissions {
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
        }
    }
}

fn get_role_from_row_wrapped(row_wrapped: Result<Option<Row>, Error>)
    -> Result<Role, DBError> {
    match row_wrapped {
        Ok (row_option) => {
            match row_option {
                Some(row_ref) => Ok (get_role_from_row(row_ref)),
                None => Err(DBError::NotFound),
            }
        }

        Err (e) => {
            error!("Error while getting rendition from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

fn get_roles_from_rows_wrapped(row_wrapped: Result<Vec<Row>, Error>)
    -> Result<Vec<Role>, DBError> {
    match row_wrapped {
        Ok (rows) => {
            let mut roles: Vec<Role> = vec![];

            for row_ref in rows.iter() {
                let row = row_ref.clone();

                roles.push(get_role_from_row(row));
            }

            Ok (roles)
        }

        Err (e) => {
            error!("Error while getting rendition from query: {}", e);

            Err(DBError::OtherError)
        }
    }
}

impl RoleRepository for MySQLRoleRepository {
    fn get(&mut self, id: u8) -> Result<Role, DBError> {
        get_role_from_row_wrapped(self.get_row(
            r"SELECT
                ID, ROLE_NAME, CREATE_IMAGE, READ_IMAGE, MODIFY_IMAGE,
                DELETE_IMAGE, READ_RENDITIONS, CREATE_RENDITIONS,
                MODIFY_RENDITIONS, DELETE_RENDITIONS, READ_PROJECT,
                CREATE_PROJECT, MODIFY_PROJECT, DELETE_PROJECT,
                READ_USER, CREATE_USER, MODIFY_USER, DELETE_USER,
                PUBLISH, PUBLISH_ALL, ACCESS_ALL_PROJECTS
            FROM USER_ROLE where ID = :id",
            params! { "id" => id }
        ))
    }

    fn get_all(&mut self) -> Result<Vec<Role>, DBError> {
        get_roles_from_rows_wrapped(self.get_rows(
            r"SELECT
                ID, ROLE_NAME, CREATE_IMAGE, READ_IMAGE, MODIFY_IMAGE,
                DELETE_IMAGE, READ_RENDITIONS, CREATE_RENDITIONS,
                MODIFY_RENDITIONS, DELETE_RENDITIONS, READ_PROJECT,
                CREATE_PROJECT, MODIFY_PROJECT, DELETE_PROJECT, READ_USER,
                CREATE_USER, MODIFY_USER, DELETE_USER, PUBLISH, PUBLISH_ALL,
                ACCESS_ALL_PROJECTS
            FROM USER_ROLE",
            Params::Empty
        ))
    }

    fn add(&mut self, role: Role) -> Result<String, String> {
        match &self.connection.exec_drop(
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
                "access_all_projects" => &role.permissions.access_all_projects,
            }
        ) {
            Ok(_) => Ok(String::from("Successfully created new role!")),

            Err (e) => {
                error!("Error saving role: {}", e);

                Err(String::from("Error saving role."))
            }
        }
    }

    fn update(&mut self, role: Role) -> Result<String, String> {
        match &self.connection.exec_drop(r"UPDATE USER_ROLE SET
                ROLE_NAME = :role_name, CREATE_IMAGE = :create_image,
                READ_IMAGE = :read_image, MODIFY_IMAGE = :modify_image,
                DELETE_IMAGE = :delete_image,
                READ_RENDITIONS = :read_renditions,
                CREATE_RENDITIONS = :create_renditions,
                MODIFY_RENDITIONS = :modify_renditions,
                DELETE_RENDITIONS = :delete_renditions,
                READ_PROJECT = :read_project, CREATE_PROJECT = :create_project,
                MODIFY_PROJECT = :modify_project,
                DELETE_PROJECT = :delete_project,
                READ_USER = :read_user, CREATE_USER = :create_user,
                MODIFY_USER = :modify_user, DELETE_USER = :delete_user,
                PUBLISH = :publish, PUBLISH_ALL = :publish_all,
                ACCESS_ALL_PROJECTS = :access_all_projects
            WHERE ID = :id",
            params! {
                "id" => &role.id,
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
                "access_all_projects" => &role.permissions.access_all_projects,
            }
        ) {
            Ok(_) => Ok(String::from("Successfully updated role!")),

            Err (e) => {
                error!("Error while updating role: {}", e);

                Err(String::from("Unable to update role."))
            }
        }
    }

    fn remove(&mut self, role: Role) -> Result<String, String> {
        self.remove_item(role.id as u32)
    }

    fn remove_item(&mut self, id: u32) -> Result<String, String> {
        match &self.connection.exec_drop(
            r"DELETE FROM USER_ROLE WHERE ID = :id",
            params! { "id" => id.clone() }) {

            Ok (_) => {
                info!("Role with ID: {} removed successfully!", id);

                Ok (String::from("Successfully removed role."))
            }

            Err (e) => {
                error!("Unable to remove role with ID: {}\nError: {}", id, e);

                Err (String::from("Unable to remove role."))
            }
        }
    }
}

impl MySQLRoleRepository {
    fn get_row(&mut self, query: &str, params: Params)
        -> mysql::error::Result<Option<Row>> {
        get_row_from_query(&mut self.connection, query, params)
    }

    fn get_rows(&mut self, query: &str, params: Params)
        -> mysql::error::Result<Vec<Row>> {
        get_rows_from_query(&mut self.connection, query, params)
    }
}

