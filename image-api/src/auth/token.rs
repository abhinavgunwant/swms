use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize)]
pub struct RefreshToken {
    pub refresh_token: String,
    login_id: String,
    expire: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct SessionToken {
    login_id: String,
}

static mut REFRESH_TOKENS: Vec<RefreshToken> = vec![];

pub unsafe fn store_refresh_token(token: RefreshToken) {
    REFRESH_TOKENS.push(token);
}

pub unsafe fn refresh_token_exists(token: String) -> bool {
    for (i, t) in REFRESH_TOKENS.iter().enumerate() {
        if t.refresh_token.eq(&token) {
            if Utc::now() < t.expire {
                return true;
            } else {
                REFRESH_TOKENS.remove(i);
            }
        }
    }

    false
}

/**
 * Returns a refresh token that will be used in case the user "refresh"es the
 * page to re-issue the session token (jwt).
 * 
 * TODO: Right now, it only returns user's login id, change it to return
 * a crypto-random string that can be used to uniquely identify the user
 * session.
 */
pub fn create_refresh_token(login_id: String) -> String {
//    RefreshToken {
//        refresh_token: String::from("test"),
//        login_id,
//        expire: Utc::now()
//    }
    login_id
}

/**
 * Generates JWT token string
 * 
 * TODO: modify to create a jwt token
 */
pub fn create_session_token (login_id: String) -> String {
    login_id
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
