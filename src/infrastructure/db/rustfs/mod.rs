use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials};

use crate::infrastructure::config::rustfs_server_config::RustFSServerConfig;

pub struct RustFSClient {
    pub rustfs_client: Client,
}

impl RustFSClient {
    pub async fn new(rustfs_server_config: RustFSServerConfig) -> Self {
        let credentials = Credentials::new(
            rustfs_server_config.rustfs_access_key_id,
            rustfs_server_config.rustfs_secret_access_key,
            None,
            None,
            "rustfs",
        );
        let region = Region::new(rustfs_server_config.rustfs_region);
        let endpoint_url = rustfs_server_config.rustfs_endpoint_url;
        let shard_config = aws_config::defaults(BehaviorVersion::latest())
            .region(region)
            .credentials_provider(credentials)
            .endpoint_url(endpoint_url)
            .load()
            .await;
        RustFSClient {
            rustfs_client: Client::new(&shard_config),
        }
    }
}
