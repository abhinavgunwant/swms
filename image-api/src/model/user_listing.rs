use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListing {
    pub id: u32,
    pub login_id: String,
    pub name: String,
    pub email: String,
}

