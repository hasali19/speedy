mod db;
mod handlers;
mod runner;
mod speedtest;

use std::env;
use std::sync::Arc;

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use anyhow::Result;

use db::Db;
use runner::Runner;
use speedtest::{TestClient, TestClientBuilder, TestResult};

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
    run_scheduler(Arc::clone(&runner));

    // Run web server.
    run_server(Arc::clone(&db), runner).await?;

    // Ensure database connections are closed.
    db.close().await;

    Ok(())
}

fn run_scheduler(runner: Arc<Runner>) {
    let default = |_| "0 0 * * * * *".to_owned();
    let cron = env::var("SPEEDY_CRON").unwrap_or_else(default);
    tokio::spawn(runner.run_scheduler(cron));
}

async fn run_server(db: Arc<Db>, runner: Arc<Runner>) -> Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Arc::clone(&db))
            .app_data(Arc::clone(&runner))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .route("/run_test", web::post().to(handlers::run_test))
                    .service(
                        web::resource("/results")
                            .name("results")
                            .route(web::get().to(handlers::get_results)),
                    ),
            )
            .service(Files::new("/", "./client/build").index_file("index.html"))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await?;

    Ok(())
}

fn init_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init_timed();
}

fn create_test_client() -> TestClient {
    let default = |_| "speedy".to_owned();
    let path = env::var("SPEEDY_SPEEDTEST_PATH").unwrap_or_else(default);

    let mut builder = TestClientBuilder::with_path(&path);

    if env::var_os("SPEEDY_ACCEPT_AGREEMENTS").is_some() {
        builder = builder.accept_license().accept_gdpr();
    }

    builder.build_client()
}

fn create_test_runner(client: TestClient, db: Arc<Db>) -> Arc<Runner> {
    let runner = Runner::new(client).on_success(on_test_success(db));
    Arc::new(runner)
}

async fn create_db() -> Arc<Db> {
    let default = |_| "sqlite::memory:".to_owned();
    let url = env::var("SPEEDY_DATABASE_URL").unwrap_or_else(default);
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
