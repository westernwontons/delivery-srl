use argon2::password_hash::{
    rand_core::OsRng, Error as PasswordHashError, PasswordHash, PasswordHasher, PasswordVerifier,
    SaltString
};
use argon2::Argon2;

/// Generate an `argon2` password hash from `password`
pub fn gen_password_hash(password: &str) -> Result<String, PasswordHashError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

/// Verify a password against a hash
///
/// If hashing is successful, the return type is [`Ok(true)`], otherwise [`Ok(false)`]
///
/// Otherwise, returns the error that occured when hashing the password
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordHashError> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
