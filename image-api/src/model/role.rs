use serde::{ Serialize, Deserialize };

use crate::model::user_permissions::UserPermissions;

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: u8,
    pub role_name: String,
    pub permissions: UserPermissions,
}

