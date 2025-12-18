use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Option<Thing>,
    pub user_id: String,
    pub device_name: String,
    pub device_type: String,
    pub raw_user_agent: String,
    pub first_login_at: DateTime<Utc>,
    pub last_login_at: DateTime<Utc>,
    pub ip: String,
    pub is_trusted: bool,
}
