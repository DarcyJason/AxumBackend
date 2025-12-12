use serde::{Deserialize, Serialize};

use crate::domain::user::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub username: String,
    pub role: Role,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}
