use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::db::Db;
use crate::runner::Runner;

pub async fn index() -> impl Responder {
    let index_html: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/index.html"));
    HttpResponse::Ok().body(index_html)
}

pub async fn run_test(req: HttpRequest) -> impl Responder {
    let runner: &Arc<Runner> = req.app_data().unwrap();

    // Try running a test in the background.
    // If a test is already running, return an error response.
    if runner.try_run().await {
        HttpResponse::Ok().body("Ok")
    } else {
        HttpResponse::Conflict().body("A test is already running.")
    }
}

pub async fn get_results(req: HttpRequest) -> impl Responder {
    let db: &Arc<Db> = req.app_data().unwrap();
    let results = db.get_all_results().await.unwrap();
    HttpResponse::Ok().json(results)
}
