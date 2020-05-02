use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

use async_std::sync::Mutex;
use async_std::task;

use crate::speedtest::{Client, TestResult};

pub trait SuccessFn: Fn(TestResult) + Send + Sync {}

impl<F: Fn(TestResult) + Send + Sync> SuccessFn for F {}

pub struct Runner {
    client: Client,
    is_running: Mutex<bool>,
    on_success: Option<Box<dyn SuccessFn>>,
}

impl Runner {
    pub fn new(client: Client) -> Runner {
        Runner {
            client,
            is_running: Mutex::new(false),
            on_success: None,
        }
    }

    pub fn on_success(mut self, on_success: impl SuccessFn + 'static) -> Runner {
        self.on_success = Some(Box::new(on_success));
        self
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
}

fn run_test(runner: &Arc<Runner>) -> impl Future<Output = ()> {
    let runner = Arc::clone(&runner);
    async move {
        let result = runner.client.run_test();

        runner.set_idle().await;

        match result {
            Ok(result) => {
                if let Some(ref on_success) = runner.on_success {
                    on_success(result);
                }
            }
            Err(e) => log::error!("Test failed: {}", e),
        }
    }
}

pub async fn try_run(runner: &Arc<Runner>) -> bool {
    if runner.set_running().await {
        task::spawn(run_test(runner));
        true
    } else {
        false
    }
}

pub async fn run_scheduler(runner: Arc<Runner>) {
    loop {
        log::info!("Running test...");

        if !try_run(&runner).await {
            log::info!("A test is already running, skipping scheduled");
        }

        log::info!("Next test scheduled for 5 minutes from now");

        task::sleep(Duration::from_secs(360)).await;
    }
}
