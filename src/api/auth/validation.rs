use validator::ValidateEmail;

use crate::{
    app::{errors::validation_error::ValidationErrorKind, result::AppResult},
    domain::request::auth::{LoginRequest, RegisterRequest},
    utils::regex::{NAME_REGEX, PASSWORD_REGEX},
};

pub fn validate_register_payload(payload: &RegisterRequest) -> AppResult<()> {
    if payload.name.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("name can't be empty".to_string()).into(),
        );
    }
    if payload.name.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed("name is too long".to_string()).into());
    }
    if !NAME_REGEX.is_match(&payload.name).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed("name is invalid".to_string()).into());
    }
    if payload.email.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("email can't be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed("email is invalid".to_string()).into());
    }
    if payload.password.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("password can't be empty".to_string()).into(),
        );
    }
    if payload.password.len() < 8 {
        return Err(
            ValidationErrorKind::ValidationFailed("password is too short".to_string()).into(),
        );
    }
    if payload.password.len() > 20 {
        return Err(
            ValidationErrorKind::ValidationFailed("password is too long".to_string()).into(),
        );
    }
    if !PASSWORD_REGEX.is_match(&payload.password).unwrap_or(false) {
        return Err(
            ValidationErrorKind::ValidationFailed("password is invalid".to_string()).into(),
        );
    }
    if payload.confirm_password.is_empty() {
        return Err(ValidationErrorKind::ValidationFailed(
            "confirm_password can't be empty".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "confirm_password is too short".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "confirm_password is too long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.confirm_password)
        .unwrap_or(false)
    {
        return Err(ValidationErrorKind::ValidationFailed(
            "confirm_password is invalid".to_string(),
        )
        .into());
    }
    if payload.password != payload.confirm_password {
        return Err(
            ValidationErrorKind::ValidationFailed("password doesn't match".to_string()).into(),
        );
    }
    Ok(())
}

pub fn validate_login_payload(payload: &LoginRequest) -> AppResult<()> {
    if payload.email.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("email can't be empty".to_string()).into(),
        );
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(ValidationErrorKind::ValidationFailed("email is invalid".to_string()).into());
    }
    if payload.password.is_empty() {
        return Err(
            ValidationErrorKind::ValidationFailed("password can't be empty".to_string()).into(),
        );
    }
    if payload.password.len() < 8 {
        return Err(
            ValidationErrorKind::ValidationFailed("password is too short".to_string()).into(),
        );
    }
    if payload.password.len() > 20 {
        return Err(
            ValidationErrorKind::ValidationFailed("password is too long".to_string()).into(),
        );
    }
    if !PASSWORD_REGEX.is_match(&payload.password).unwrap_or(false) {
        return Err(
            ValidationErrorKind::ValidationFailed("password is invalid".to_string()).into(),
        );
    }
    Ok(())
}
