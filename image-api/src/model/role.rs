use serde::Serialize;

use crate::model::user_permissions::UserPermissions;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    pub id: u16,
    pub role_name: String,
    pub permissions: UserPermissions,
}

