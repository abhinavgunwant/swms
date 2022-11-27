use serde::Serialize;

#[derive(Serialize)]
pub struct UserSearch {
    pub id: u32,
    pub name: String,
}

