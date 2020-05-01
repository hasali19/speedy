mod db;
mod runner;
mod speedtest;

use db::Db;
use runner::Runner;
use speedtest::{Client as TestClient, TestResult};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load env vars from .env file.
    dotenv::dotenv().ok();

    let db = Arc::new(Db::new("sqlite::memory:").await.unwrap());

    let client = create_test_client();
    let runner = Runner::create(client);

    // Run test scheduler loop.
    Runner::run_scheduler(runner, on_scheduled_test_success(db)).await;
}

fn create_test_client() -> TestClient {
    let default_path = "speedy".to_owned();
    let path = std::env::var("SPEEDY_SPEEDTEST_PATH").unwrap_or(default_path);
    TestClient::from_path(&path)
}

fn on_scheduled_test_success(db: Arc<Db>) -> impl Fn(TestResult) {
    move |result| {
        let db = db.clone();

        // Save result to database.
        tokio::spawn(async move {
            db.create_result(&result.into()).await.unwrap();
        });
    }
}
