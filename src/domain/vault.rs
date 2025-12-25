use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub user_id: Thing,
    pub ciphers: Vec<Thing>,
    pub folders: Vec<Thing>,
    pub organizations: Vec<Thing>,
}
