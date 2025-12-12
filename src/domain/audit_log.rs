use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Option<Thing>,
    pub user_id: Option<String>,
    pub action: String,
    pub status: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub detail: Option<String>,
    pub created_at: DateTime<Utc>,
}
