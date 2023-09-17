use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;
use jsonwebtoken::{ encode, EncodingKey, Header };
use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc, Duration };
use rand::{
    SeedableRng, rngs::{ OsRng, adapter::ReseedingRng },
    distributions::{Alphanumeric, DistString},
};
use rand_chacha::ChaCha20Core;

use crate::model::role::Role;

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenData {
    pub user_id: u32,
    pub username: String,
    pub name: String,
    pub role_id: u8,
    pub expiry: usize,  // timestamp of expiry
}

// JWT Claims for session
#[derive(Serialize, Deserialize)]
pub struct SessionTokenClaims {
    pub sub: String,
    pub name: String,
    pub role: Role,
    pub iat: usize,
    pub exp: usize,  // timestamp of expiry
}

lazy_static! {
    static ref REFRESH_TOKEN_MAP: Mutex<HashMap<String, RefreshTokenData>>
        = Mutex::new(HashMap::new());
}

pub unsafe fn store_refresh_token(token: String, data: RefreshTokenData) {
    REFRESH_TOKEN_MAP.lock().unwrap().insert(token, data);
}

pub unsafe fn remove_refresh_token(token: String) {
    REFRESH_TOKEN_MAP.lock().unwrap().remove(&token);
}

pub unsafe fn refresh_token_exists(token: String) -> bool {
    REFRESH_TOKEN_MAP.lock().unwrap().contains_key(&token)
}

/// Returns a refresh token that will be used in case the user "refresh"es the
/// page to re-issue the session token (jwt).
pub fn create_refresh_token(user_id: u32, username: String, name: String, role_id: u8) -> String {
    println!("Generating refresh token");

    let prng = ChaCha20Core::from_entropy();
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng);

    let token: String;

    let data = RefreshTokenData {
        user_id,
        role_id,
        username,
        name,
        expiry: (Utc::now() + Duration::minutes(30)).timestamp() as usize
    };

    let mut loop_counter: u8 = 0;

    loop {
        let t = Alphanumeric.sample_string(&mut reseeding_rng, 64);

        unsafe {
            if !refresh_token_exists(t.clone()) {
                store_refresh_token(t.clone(), data);
                token = t;
                break;
            }
        }

        if loop_counter > 98 {
            // after looping 100 times, just give up!
            return String::from("Failed at creating cookie!");
        }

        loop_counter += 1;
    }

    println!("-> token_string: {}", token);

    token
}

/// Generates JWT token string
pub fn create_session_token (username: String, name: String, role: Role) -> String {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(5)).timestamp() as usize;

    let claims: SessionTokenClaims = SessionTokenClaims {
        sub: username.clone(),
        name,
        role,
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"kuch bhi"),
    ).unwrap();

    println!("-> session token: {}", token);

    token
}

/**
 * Return session token from the supplied refresh token.
 * 
 * TODO: Change it based on the refresh token.
 */
pub fn create_session_token_from_refresh (refresh_token: String) -> String {
    refresh_token
}

// TODO: modify to verify a jwt token
pub fn verify_session_token (_token: String) -> bool {
    true
}

