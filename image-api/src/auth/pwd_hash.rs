use lazy_static::lazy_static;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Algorithm, Version, Params
};

lazy_static! {
    static ref HASHER: Argon2<'static> = Argon2::new(
        Algorithm::Argon2id,// Algorithm: Argon2id
        Version::V0x13,     // Version: 19
        Params::new(
            16384,      // m = 16MB
            2,          // t = 2
            1,          // p = 1
            Some(64)    // Output size in bytes
        ).unwrap()
    );
}

/**
 * Returns the password hash.
 */
pub fn generate_password_hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    HASHER.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(hash.as_str()).unwrap();

    HASHER.verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

