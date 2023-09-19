use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::cookie::{ time::Duration as ActixWebDuration, Cookie};
use lazy_static::lazy_static;
use jsonwebtoken::{
    encode, decode, EncodingKey, DecodingKey, Header, Validation
};
use serde::{ Serialize, Deserialize };
use chrono::{ Utc, Duration };
use rand::{
    SeedableRng, rngs::{ OsRng, adapter::ReseedingRng },
    distributions::{Alphanumeric, DistString},
};
use rand_chacha::ChaCha20Core;

use crate::{
    model::role::Role,
    repository::role::{ get_role_repository, RoleRepository }, db::DBError,
};

#[derive(Serialize, Deserialize, Clone)]
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

pub enum TokenError {
    InvalidToken,
    UserNotFound,

    /// User may have been found but is not assigned a role.
    RoleNotFound,
    OtherError,
}

lazy_static! {
    static ref REFRESH_TOKEN_MAP: Mutex<HashMap<String, RefreshTokenData>>
        = Mutex::new(HashMap::new());
}

const JWT_SECRET: &[u8] = b"a0190as9fuhsjkfhalh";

pub unsafe fn store_refresh_token(token: String, data: RefreshTokenData) {
    REFRESH_TOKEN_MAP.lock().unwrap().insert(token, data);
}

pub unsafe fn remove_refresh_token(token: String) {
    REFRESH_TOKEN_MAP.lock().unwrap().remove(&token);
}

pub unsafe fn refresh_token_exists(token: String) -> bool {
    REFRESH_TOKEN_MAP.lock().unwrap().contains_key(&token)
}

pub unsafe fn get_refresh_token(token: String) -> Option<RefreshTokenData> {
    match REFRESH_TOKEN_MAP.lock() {
        Ok(locked_ref_token_map) => {
            match locked_ref_token_map.get(&token) {
                Some(ref_token_data) => Some(ref_token_data.clone()),
                None => None,
            }
        }
        Err(_) => None,
    }
}

pub unsafe fn update_refresh_token_expiry(token: String) {
    match REFRESH_TOKEN_MAP.lock() {
        Ok(mut ref_hm) => {
            match ref_hm.get_mut(&token) {
                Some(ref_tok_data) => {
                    ref_tok_data.expiry = get_expiry_from_now();
                }

                None => {}
            };
        }

        Err(e) => { eprintln!("Error: {}", e); }
    }
}

fn get_expiry_from_now() -> usize {
    (Utc::now() + Duration::minutes(30)).timestamp() as usize
}

fn encode_jwt(claims: &SessionTokenClaims) -> String {
    // TODO: get this from a config file or environment.

    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ) {
        Ok (t) => t,

        Err (e) => {
            eprintln!("Error while generating jwt: {}", e);

            return String::from("");
        }
    }
}

pub fn decode_jwt(token: &String) -> Result<SessionTokenClaims, ()> {
    match decode::<SessionTokenClaims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    ) {
        Ok(claims) => Ok(claims.claims),

        Err(e) => {
            eprintln!("Error while decoding jwt: {}", e);

            return Err(());
        }
    }
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
        expiry: get_expiry_from_now()
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
pub fn create_session_token(username: String, name: String, role: Role) -> String {
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

    let token = encode_jwt(&claims);

    println!("-> session token: {}", token);

    token
}

/// Returns session token from the supplied refresh token.
pub fn create_session_token_from_refresh_token (refresh_token: String)
    -> Result<String, TokenError> {

    if refresh_token.is_empty() {
        return Err(TokenError::InvalidToken);
    }

    unsafe {
        match get_refresh_token(refresh_token) {
            Some(refresh_data) => {
                match get_role_repository().get(refresh_data.role_id) {
                    Ok(role) => {
                        let session_token = create_session_token(
                            refresh_data.username.clone(),
                            refresh_data.name.clone(),
                            role
                        );

                        if !session_token.is_empty() {
                            update_refresh_token_expiry(session_token.clone());

                            return Ok(session_token);
                        }

                        return Err(TokenError::OtherError);
                    }

                    Err(e) => {
                        match e {
                            DBError::NOT_FOUND => { return Err(TokenError::RoleNotFound); }
                            DBError::OtherError => { return Err(TokenError::OtherError); }
                        }
                    }
                }
            }

            None => { return Err(TokenError::InvalidToken); }
        }
    }
}

pub fn create_refresh_token_cookie<'a>(refresh_token: String) -> Cookie<'a> {
    Cookie::build("r", refresh_token)
        .path("/")
        .domain("localhost") // TODO: make this configurable
        // .secure(true) // TODO: uncomment this for secure cookie!
        .max_age(ActixWebDuration::new(1800, 0))
        .http_only(true)
        .finish()
}

// TODO: modify to verify a jwt token
pub fn verify_session_token (_token: String) -> bool {
    true
}

