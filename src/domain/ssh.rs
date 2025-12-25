use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSH {
    pub id: Thing,
    pub secret_key: Option<String>,
    pub public_key: Option<String>,
    pub fingerprint: Option<String>,
}
