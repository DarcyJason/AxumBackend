use validator::ValidateEmail;

use crate::{
    app::{errors::validation_error::ValidationErrorKind, result::AppResult},
    domain::request::auth::{LoginRequest, RegisterRequest},
    utils::regex::{NAME_REGEX, PASSWORD_REGEX},
};

pub fn validate_register_payload(payload: &RegisterRequest) -> AppResult<()> {
    if payload.name.len() < 2 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Name should be at least 2 characters long".to_string(),
        )
        .into());
    }
    if payload.name.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Name should be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !NAME_REGEX.is_match(&payload.name).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed("Invalid name".to_string()).into());
    }
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(
            ValidationErrorKind::ValidationFailed("Invalid email address".to_string()).into(),
        );
    }
    if payload.password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password should be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password should be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX.is_match(&payload.password).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed("Invalid password".to_string()).into());
    }
    if payload.confirm_password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password should be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.confirm_password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Confirm password should be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX
        .is_match(&payload.confirm_password)
        .unwrap_or(false)
    {
        return Err(
            ValidationErrorKind::ValidationFailed("Invalid confirm password".to_string()).into(),
        );
    }
    if payload.password != payload.confirm_password {
        return Err(
            ValidationErrorKind::ValidationFailed("Passwords do not match".to_string()).into(),
        );
    }
    Ok(())
}

pub fn validate_login_payload(payload: &LoginRequest) -> AppResult<()> {
    if !ValidateEmail::validate_email(&payload.email) {
        return Err(
            ValidationErrorKind::ValidationFailed("Invalid email address".to_string()).into(),
        );
    }
    if payload.password.len() < 8 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password should be at least 8 characters long".to_string(),
        )
        .into());
    }
    if payload.password.len() > 20 {
        return Err(ValidationErrorKind::ValidationFailed(
            "Password should be at most 20 characters long".to_string(),
        )
        .into());
    }
    if !PASSWORD_REGEX.is_match(&payload.password).unwrap_or(false) {
        return Err(ValidationErrorKind::ValidationFailed("invalid Password".to_string()).into());
    }
    Ok(())
}
