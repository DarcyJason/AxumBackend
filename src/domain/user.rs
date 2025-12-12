use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Thing>,
    pub name: String,
    pub email: String,
    pub password_hashed: String,
    pub role: Role,
    pub is_verified: bool,
    pub is_banned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Role {
    User,
    Admin,
    SystemOwner,
}
