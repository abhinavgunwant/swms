use chrono::{ DateTime, Utc };

pub struct RefreshToken {
    refresh_token: String,
    login_id: String,
    expire: DateTime<Utc>
}

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
