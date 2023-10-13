use actix_web::cookie::{ time::Duration as ActixWebDuration, Cookie };
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
use log::{ debug, error };

use crate::{
    model::role::Role,
    repository::role::RoleRepository, db::DBError,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct RefreshTokenData {
    pub user_id: u32,
    pub username: String,
    pub name: String,
    pub role_id: u8,
    pub expiry: usize,  // timestamp of expiry
}

/// JWT Claims for session
#[derive(Serialize, Deserialize)]
pub struct SessionTokenClaims {
    pub sub: String,
    pub name: String,
    pub id: u32,
    pub role: Role,
    pub iat: usize,
    pub exp: usize,  // timestamp of expiry
}

pub enum TokenError {
    //InvalidToken,
    // UserNotFound,

    /// User may have been found but is not assigned a role.
    RoleNotFound,
    OtherError,
}

const JWT_SECRET: &[u8] = b"a0190as9fuhsjkfhalh";
const REF_TOKEN_LENGTH: usize = 64;

/// Returns time n minutes from now. `n` is the session timeout time.
pub fn get_expiry_from_now() -> usize {
    (Utc::now() + Duration::minutes(30)).timestamp() as usize
}

pub fn encode_jwt(claims: &SessionTokenClaims) -> String {
    match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET), // TODO: get this from a config file or environment.
    ) {
        Ok (t) => t,

        Err (e) => {
            error!("Error while generating jwt: {}", e);

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
            error!("Error while decoding jwt: {}", e);

            return Err(());
        }
    }
}

/// Returns a unique string used as the refresh token cookie value.
pub fn create_refresh_token() -> String {
    debug!("Generating refresh token");

    let prng = ChaCha20Core::from_entropy();
    let mut reseeding_rng = ReseedingRng::new(prng, 0, OsRng);

    let token = Alphanumeric.sample_string(
        &mut reseeding_rng,
        REF_TOKEN_LENGTH
    );

    debug!("Generated token: {}", token);

    token
}

/// Generates JWT token string
pub fn create_session_token(
    username: String, name: String, id: u32, role: Role
) -> String {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(5)).timestamp() as usize;

    let claims: SessionTokenClaims = SessionTokenClaims {
        sub: username.clone(),
        name,
        id,
        role,
        iat,
        exp,
    };

    let token = encode_jwt(&claims);

    debug!("-> session token: {}", token);

    token
}

/// Returns session token from the supplied refresh token.
pub fn create_session_token_from_refresh_token(
    ref_tok_data: RefreshTokenData,
    role_repo: &mut Box<dyn RoleRepository>,
) -> Result<String, TokenError> {
    match role_repo.get(ref_tok_data.role_id) {
        Ok(role) => {
            let session_token = create_session_token(
                ref_tok_data.username.clone(),
                ref_tok_data.name.clone(),
                ref_tok_data.user_id,
                role
            );

            if !session_token.is_empty() {
                return Ok(session_token);
            }

            return Err(TokenError::OtherError);
        }

        Err(e) => {
            match e {
                DBError::NOT_FOUND => Err(TokenError::RoleNotFound),
                _ => Err(TokenError::OtherError),
            }
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

