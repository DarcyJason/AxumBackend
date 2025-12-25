use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    domain::{
        access_token_claims::AccessTokenClaims, refresh_token_claims::RefreshTokenClaims,
        user::Role,
    },
};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use surrealdb::sql::Thing;
use uuid::Uuid;

pub fn encode_access_token(
    user_id: Thing,
    username: String,
    role: Role,
    secret: &str,
    expires_in_seconds: i64,
) -> AppResult<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(expires_in_seconds)).timestamp() as usize;
    let jti = Uuid::new_v4().to_string();
    let access_token_claims = AccessTokenClaims {
        sub: user_id,
        username,
        role,
        exp,
        iat,
        jti,
    };
    Ok(encode(
        &Header::new(Algorithm::HS256),
        &access_token_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(ExternalError::from)?)
}

pub fn decode_access_token(access_token: &str, secret: &str) -> AppResult<AccessTokenClaims> {
    let access_token = access_token.to_string();
    Ok(decode::<AccessTokenClaims>(
        &access_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(ExternalError::from)?
    .claims)
}

pub fn encode_refresh_token(
    user_id: Thing,
    secret: &str,
    expires_in_seconds: i64,
) -> AppResult<String> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::seconds(expires_in_seconds)).timestamp() as usize;
    let rtid = Uuid::new_v4().to_string();
    let refresh_token_claims = RefreshTokenClaims {
        sub: user_id,
        rtid,
        exp,
        iat,
    };
    Ok(encode(
        &Header::new(Algorithm::HS256),
        &refresh_token_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(ExternalError::from)?)
}

pub fn decode_refresh_token(refresh_token: &str, secret: &str) -> AppResult<RefreshTokenClaims> {
    let refresh_token = refresh_token.to_string();
    Ok(decode::<RefreshTokenClaims>(
        &refresh_token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(ExternalError::from)?
    .claims)
}
