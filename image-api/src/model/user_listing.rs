use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize )]
#[serde(rename_all = "camelCase")]
pub struct UserListing {
    pub id: u32,
    pub login_id: String,
    pub name: String,
    pub email: String,
}

