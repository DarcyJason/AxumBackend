use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
    pub id: Thing,
    pub username: Option<String>,
    pub password: Option<String>,
    pub web_url: Option<String>,
}
