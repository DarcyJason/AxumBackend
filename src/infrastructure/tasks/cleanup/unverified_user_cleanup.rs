use std::{sync::Arc, time::Duration};

use crate::app::state::AppState;

pub async fn start(app_state: Arc<AppState>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;
            match app_state.services.user.unverified_user_cleanup().await {
                Ok(_) => {
                    tracing::info!("unverified users cleanup completed");
                }
                Err(e) => {
                    tracing::error!("unverified user cleanup failed: {}", e);
                }
            }
        }
    });
}
