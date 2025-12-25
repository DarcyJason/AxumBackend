use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Thing,
    pub rtid: String,
    pub exp: usize,
    pub iat: usize,
}
