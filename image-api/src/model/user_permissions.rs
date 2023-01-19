use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPermissions {
    pub create_image: bool,
    pub read_image: bool,
    pub modify_image: bool,
    pub delete_image: bool,
    pub read_renditions: bool,
    pub create_renditions: bool,
    pub modify_renditions: bool,
    pub delete_renditions: bool,
    pub read_project: bool,
    pub create_project: bool,
    pub modify_project: bool,
    pub delete_project: bool,
    pub read_user: bool,
    pub create_user: bool,
    pub modify_user: bool,
    pub delete_user: bool,
    pub publish: bool,
    pub publish_all: bool,
    pub access_all_projects: bool,
}

