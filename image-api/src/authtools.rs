use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Algorithm, Version, Params
};

/**
 * Returns an `Argon2` context initialized with parameters recommended by owasp
 * in: https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
 */
fn get_password_hasher<'a>() -> Argon2<'a> {
    Argon2::new(
        Algorithm::Argon2id,// Algorithm: Argon2id
        Version::V0x13,     // Version: 19
        Params::new(
            16384,      // m = 16MB
            2,          // t = 2
            1,          // p = 1
            Some(64)    // Output size in bytes
        ).unwrap()
    )
}

/**
 * Returns the password hash using the Argon2id algorithm.
 */
pub fn generate_password_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    get_password_hasher()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}
