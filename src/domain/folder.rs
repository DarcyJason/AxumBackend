use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

pub struct Folder {
    pub id: Thing,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
