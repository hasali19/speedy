mod db;
mod routes;
mod runner;
mod speedtest;

use std::sync::Arc;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use anyhow::Result;

use db::Db;
use runner::Runner;
use speedtest::{Client as TestClient, TestResult};

#[actix_rt::main]
async fn main() -> Result<()> {
    // Load env vars from .env file.
    dotenv::dotenv().ok();

    init_logger();

    // Connect to database.
    let db = create_db().await;

    let client = create_test_client();
    let runner = create_test_runner(client, Arc::clone(&db));

    // Run test scheduler loop.
    tokio::spawn(runner::run_scheduler(Arc::clone(&runner)));

    // Run web server.
    run_server(Arc::clone(&db), runner).await?;

    // Ensure database connections are closed.
    db.close().await;

    Ok(())
}

async fn run_server(db: Arc<Db>, runner: Arc<Runner>) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Arc::clone(&db))
            .app_data(Arc::clone(&runner))
            .wrap(Logger::default())
            .route("/", web::get().to(routes::index))
            .service(
                web::scope("/api")
                    .route("/run_test", web::post().to(routes::run_test))
                    .route("/results", web::get().to(routes::get_results)),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}

fn init_logger() {
    if let None = std::env::var_os("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init_timed();
}

fn create_test_client() -> TestClient {
    let default_path = "speedy".to_owned();
    let path = std::env::var("SPEEDY_SPEEDTEST_PATH").unwrap_or(default_path);
    TestClient::from_path(&path)
}

fn create_test_runner(client: TestClient, db: Arc<Db>) -> Arc<Runner> {
    let runner = Runner::new(client).on_success(on_test_success(db));
    Arc::new(runner)
}

async fn create_db() -> Arc<Db> {
    let default_url = "sqlite::memory:".to_owned();
    let url = std::env::var("SPEEDY_DATABASE_URL").unwrap_or(default_url);
    Arc::new(Db::new(&url).await.unwrap())
}

fn on_test_success(db: Arc<Db>) -> impl Fn(TestResult) {
    move |result| {
        let db = db.clone();

        // Save result to database.
        tokio::spawn(async move {
            db.create_result(&result.into()).await.unwrap();
        });
    }
}
