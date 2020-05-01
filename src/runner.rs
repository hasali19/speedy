use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::speedtest::{Client, TestResult};

pub struct Runner {
    client: Arc<Client>,
    is_running: Mutex<bool>,
}

impl Runner {
    pub fn new(client: Client) -> Runner {
        Runner {
            client: Arc::new(client),
            is_running: Mutex::new(false),
        }
    }

    pub fn create(client: Client) -> Arc<Runner> {
        Arc::new(Runner::new(client))
    }

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

    #[allow(dead_code)]
    pub async fn try_run(runner: Arc<Runner>) -> Option<impl Future<Output = Result<TestResult>>> {
        if !runner.set_running().await {
            return None;
        }

        let future = async move {
            let results = runner.client.run_test().await;
            runner.set_idle().await;
            results
        };

        Some(future)
    }

    pub async fn run_scheduler(runner: Arc<Runner>) {
        let client = &runner.client;
        loop {
            if !runner.is_running().await {
                let result: TestResult = client.run_test().await.unwrap();

                println!(
                    "Ping: {}, Download: {}, Upload: {}",
                    result.ping.latency,
                    (result.download.bandwidth as f64) * 8.0 / 1000.0 / 1000.0,
                    (result.upload.bandwidth as f64) * 8.0 / 1000.0 / 1000.0
                );
            }

            tokio::time::delay_for(Duration::from_secs(60)).await;
        }
    }
}
