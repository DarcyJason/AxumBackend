use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOwnerConfig {
    pub system_owner_name: String,
    pub system_owner_email: String,
    pub system_owner_password: String,
}
