use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::{
    app::{errors::external_error::ExternalError, result::AppResult},
    infrastructure::config::surreal_server_config::SurrealServerConfig,
};

#[derive(Debug, Clone)]
pub struct SurrealClient {
    pub client: Surreal<Client>,
}

impl SurrealClient {
    pub async fn new(surreal_server_config: SurrealServerConfig) -> AppResult<Self> {
        let db = Surreal::new::<Ws>(surreal_server_config.surreal_host)
            .await
            .map_err(ExternalError::from)?;
        db.signin(Root {
            username: &surreal_server_config.surreal_root_name,
            password: &surreal_server_config.surreal_root_password,
        })
        .await
        .map_err(ExternalError::from)?;
        db.use_ns(surreal_server_config.surreal_namespace)
            .use_db(surreal_server_config.surreal_database)
            .await
            .map_err(ExternalError::from)?;
        Ok(SurrealClient { client: db })
    }
}
