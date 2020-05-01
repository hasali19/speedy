mod runner;
mod speedtest;

use runner::Runner;
use speedtest::Client as TestClient;

#[tokio::main]
async fn main() {
    let client = create_test_client();
    let runner = Runner::create(client);
    Runner::run_scheduler(runner).await;
}

fn create_test_client() -> TestClient {
    let default_path = "speedy".to_owned();
    let path = std::env::var("SPEEDY_SPEEDTEST_PATH").unwrap_or(default_path);
    TestClient::from_path(&path)
}
