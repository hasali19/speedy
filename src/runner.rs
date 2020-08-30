use std::sync::{Arc, RwLock};

use chrono::Local;
use cron::Schedule;

use crate::speedtest::{TestClient, TestResult};

pub trait SuccessFn: Fn(TestResult) + Send + Sync {}

impl<F: Fn(TestResult) + Send + Sync> SuccessFn for F {}

pub struct Runner {
    client: TestClient,
    is_running: RwLock<bool>,
    on_success: Option<Box<dyn SuccessFn>>,
}

impl Runner {
    pub fn new(client: TestClient) -> Runner {
        Runner {
            client,
            is_running: RwLock::new(false),
            on_success: None,
        }
    }

    pub fn on_success(mut self, on_success: impl SuccessFn + 'static) -> Runner {
        self.on_success = Some(Box::new(on_success));
        self
    }

    #[allow(dead_code)]
    pub fn is_running(&self) -> bool {
        *self.is_running.read().unwrap()
    }

    fn set_running(&self) -> bool {
        let mut is_running = self.is_running.write().unwrap();
        if *is_running {
            false
        } else {
            *is_running = true;
            true
        }
    }

    fn set_idle(&self) {
        *self.is_running.write().unwrap() = false;
    }

    async fn run_test(self: Arc<Runner>) {
        let result = self.client.run_test().await;

        self.set_idle();

        match result {
            Ok(result) => {
                if let Some(ref on_success) = self.on_success {
                    log::debug!("Test successful: {:?}", result);
                    on_success(result);
                }
            }
            Err(e) => log::error!("Test failed: {}", e),
        }
    }

    pub async fn try_run(self: &Arc<Runner>) -> bool {
        if self.set_running() {
            tokio::spawn(Arc::clone(self).run_test());
            true
        } else {
            false
        }
    }

    pub async fn run_scheduler(self: Arc<Runner>, cron_expression: String) {
        let schedule: Schedule = cron_expression.parse().unwrap();

        for time in schedule.upcoming(Local) {
            let now = Local::now();
            let duration: chrono::Duration = time - now;

            log::info!("Next test scheduled for {}", time);
            tokio::time::delay_for(duration.to_std().unwrap()).await;

            log::info!("Running test...");

            if !self.try_run().await {
                log::info!("A test is already running, skipping scheduled");
            }
        }
    }
}
