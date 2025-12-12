use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub rtid: String,
    pub exp: usize,
    pub iat: usize,
}
