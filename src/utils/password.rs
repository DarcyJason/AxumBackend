use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;

use crate::app::errors::validation_error::ValidationErrorKind;
use crate::app::result::AppResult;

pub fn hash_password(password: String) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hashed = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(ValidationErrorKind::PasswordHashingError)?
        .to_string();
    Ok(password_hashed)
}

pub fn compare_hashed_password(password: &str, password_hashed: &str) -> AppResult<bool> {
    let parsed_hash =
        PasswordHash::new(password_hashed).map_err(ValidationErrorKind::PasswordHashingError)?;
    let is_password_match = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok_and(|_| true);
    Ok(is_password_match)
}
