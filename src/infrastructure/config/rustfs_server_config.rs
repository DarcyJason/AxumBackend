use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustFSServerConfig {
    pub rustfs_region: String,
    pub rustfs_access_key_id: String,
    pub rustfs_secret_access_key: String,
    pub rustfs_endpoint_url: String,
}
