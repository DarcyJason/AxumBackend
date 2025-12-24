use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

pub struct Group {
    pub id: Thing,
    pub name: String,
    pub users: Vec<Thing>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
