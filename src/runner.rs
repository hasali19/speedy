use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::speedtest::{Client, TestResult};

pub struct Runner {
    client: Client,
    is_running: Mutex<bool>,
}

impl Runner {
    pub fn new(client: Client) -> Runner {
        Runner {
            client,
            is_running: Mutex::new(false),
        }
    }

    pub fn create(client: Client) -> Arc<Runner> {
        Arc::new(Runner::new(client))
    }

    #[allow(dead_code)]
    pub async fn is_running(&self) -> bool {
        *self.is_running.lock().await
    }

    async fn set_running(&self) -> bool {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            false
        } else {
            *is_running = true;
            true
        }
    }

    async fn set_idle(&self) {
        *self.is_running.lock().await = false;
    }

    async fn run_test(runner: &Arc<Runner>) -> Result<TestResult> {
        let results = runner.client.run_test().await;
        runner.set_idle().await;
        results
    }

    #[allow(dead_code)]
    pub async fn try_run(runner: Arc<Runner>) -> Option<impl Future<Output = Result<TestResult>>> {
        if !runner.set_running().await {
            return None;
        }

        Some(async move { Runner::run_test(&runner).await })
    }

    pub async fn run_scheduler(runner: Arc<Runner>, on_success: impl Fn(TestResult)) {
        loop {
            println!("Running test...");

            if runner.set_running().await {
                let result = Runner::run_test(&runner).await;
                match result {
                    Ok(result) => on_success(result),
                    Err(e) => eprintln!("Test failed: {}", e),
                }
            }

            println!("Next test scheduled for a minute from now.");

            tokio::time::delay_for(Duration::from_secs(60)).await;
        }
    }
}
