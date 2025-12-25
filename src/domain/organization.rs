use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: Thing,
    pub name: String,
    pub members: Vec<Thing>,
}
