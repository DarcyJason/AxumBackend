use tokio::signal;
use tracing::info;

pub async fn run_shutdown_tasks() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    tokio::select! {
        _ = ctrl_c => {},
    }
    println!();
    info!("✅ Signal received, starting graceful shutdown");
}
