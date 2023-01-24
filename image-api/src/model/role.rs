use serde::{ Serialize, Deserialize };

use crate::model::user_permissions::UserPermissions;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: u16,
    pub role_name: String,
    pub permissions: UserPermissions,
}

