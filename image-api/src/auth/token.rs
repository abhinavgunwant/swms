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

pub fn create_refresh_token(login_id: String) -> RefreshToken {
    RefreshToken {
        refresh_token: String::from("test"),
        login_id,
        expire: Utc::now()
    }
}

/**
 * Generates JWT token string
 * 
 * TODO: modify to create a jwt token
 */
pub fn create_session_token (login_id: String) -> String {
    login_id
}

// TODO: modify to verify a jwt token
pub fn verify_session_token (token: String) -> bool {
    true
}
