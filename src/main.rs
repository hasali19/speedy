mod db;
mod routes;
mod runner;
mod speedtest;

use std::sync::Arc;

use anyhow::Result;
use async_std::task;
use tide::Route;

use db::Db;
use runner::Runner;
use speedtest::{Client as TestClient, TestResult};

type State = (Arc<Db>, Arc<Runner>);

trait RouteExt {
    fn route(&mut self, f: impl Fn(&mut Self)) -> &mut Self;
}

impl<'a, S> RouteExt for Route<'a, S> {
    fn route(&mut self, f: impl Fn(&mut Self)) -> &mut Self {
        f(self);
        self
    }
}

fn main() -> Result<()> {
    // Load env vars from .env file.
    dotenv::dotenv().ok();

    let db = task::block_on(create_db());

    let client = create_test_client();
    let runner = Runner::create(client);

    // Run test scheduler loop.
    task::spawn(Runner::run_scheduler(
        runner.clone(),
        on_scheduled_test_success(db.clone()),
    ));

    task::block_on(run_server((db, runner)))
}

async fn run_server(state: State) -> Result<()> {
    let mut app = tide::with_state(state);

    app.at("/").get(routes::index);

    app.at("/api").route(|api| {
        api.at("/run_test").get(routes::run_test);
        api.at("/results").get(routes::get_results);
    });

    app.listen("127.0.0.1:8000").await?;

    Ok(())
}

fn create_test_client() -> TestClient {
    let default_path = "speedy".to_owned();
    let path = std::env::var("SPEEDY_SPEEDTEST_PATH").unwrap_or(default_path);
    TestClient::from_path(&path)
}

async fn create_db() -> Arc<Db> {
    let default_url = "sqlite::memory:".to_owned();
    let url = std::env::var("SPEEDY_DATABASE_URL").unwrap_or(default_url);
    Arc::new(Db::new(&url).await.unwrap())
}

fn on_scheduled_test_success(db: Arc<Db>) -> impl Fn(TestResult) {
    move |result| {
        let db = db.clone();

        // Save result to database.
        task::spawn(async move {
            db.create_result(&result.into()).await.unwrap();
        });
    }
}
