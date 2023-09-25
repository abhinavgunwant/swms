use std::{ collections::HashMap, sync::Mutex };

use crate::auth::token::{ RefreshTokenData, get_expiry_from_now };

#[derive(Default)]
pub struct ServerState {
    /// HashMap that holds all the refresh tokens and their corresponsing data.
    refresh_map: Mutex<HashMap<String, RefreshTokenData>>,
}

impl ServerState {
    pub fn get_refresh_token_data(&self, token: String)
        -> Option<RefreshTokenData> {
        match self.refresh_map.lock().unwrap().get(&token) {
            Some(ref_token_data) => Some(ref_token_data.clone()),
            None => None,
        }
    }

    pub fn insert_refresh_token(
        &self, token: String, data: RefreshTokenData
        ) {
        self.refresh_map.lock().unwrap().insert(token, data);
    }

    pub fn remove_refresh_token(&self, token: String) {
        self.refresh_map.lock().unwrap().remove(&token);
    }

    pub fn refresh_token_exists(&self, token: &String) -> bool {
        self.refresh_map.lock().unwrap().contains_key(token)
    }

    pub fn reset_refresh_token_expiry(&self, token: String) {
        match self.refresh_map.lock().unwrap().get_mut(&token) {
            Some(map_data) => { map_data.expiry = get_expiry_from_now(); }
            None => { }
        };
    }
}

