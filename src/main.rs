use backend::app::{result::AppResult, run};

#[tokio::main]
async fn main() -> AppResult<()> {
    run().await
}
