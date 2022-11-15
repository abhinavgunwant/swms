use actix_web::HttpRequest;

/**
 * Validates session token and returns user name if session is vaild.
 * In case of invalid session, an error message is returned.
 * 
 * Can be modified in future to return user object based on the username in the
 * session token...
 */
pub fn validate_session_token(req: HttpRequest) -> Result<String, String> {
    let auth_header = req.headers().get("Authorization")
        .unwrap().to_str().unwrap();

    if auth_header.chars().count() < 8 {
        return Err (String::from("Invalid Session"));
    }

    Ok(format!("{}", &auth_header[7..]))
}
