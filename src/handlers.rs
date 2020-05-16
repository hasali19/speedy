use std::sync::Arc;

use actix_web::web::Query;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db::{self, Db};
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

#[derive(Deserialize)]
pub struct ResultsListQuery {
    limit: Option<i32>,
    page: Option<i32>,
}

#[derive(Debug, Serialize)]
struct ResultsListResponse {
    results: Vec<db::TestResult>,
    meta: ResultsListResponseMeta,
}

#[derive(Debug, Serialize)]
struct ResultsListResponseMeta {
    count: i32,
    prev: Option<String>,
    next: Option<String>,
}

fn format_page_link(req: &HttpRequest, limit: i32, page: i32) -> String {
    let connection = req.connection_info();
    format!(
        "{}://{}{}?limit={}&page={}",
        connection.scheme(),
        connection.host(),
        req.path(),
        limit,
        page
    )
}

fn page_count(item_count: i32, page_size: i32) -> i32 {
    f32::ceil(item_count as f32 / page_size as f32) as i32
}

pub async fn get_results(req: HttpRequest, query: Query<ResultsListQuery>) -> impl Responder {
    let db: &Arc<Db> = req.app_data().unwrap();

    let limit = query.limit.unwrap_or(3);
    let page = query.page.unwrap_or(1);

    let count = db.get_results_count().await.unwrap();
    if page < 1 || page > page_count(count, limit) {
        // TODO: Return something better
        return HttpResponse::BadRequest().finish();
    }

    let offset = (page - 1) * limit;
    let results = db.get_results(limit, offset).await.unwrap();

    let prev = if page > 1 { Some(page - 1) } else { None };
    let next = if offset + limit < count {
        Some(page + 1)
    } else {
        None
    };

    HttpResponse::Ok().json(ResultsListResponse {
        results,
        meta: ResultsListResponseMeta {
            count,
            prev: prev.map(|p| format_page_link(&req, limit, p)),
            next: next.map(|p| format_page_link(&req, limit, p)),
        },
    })
}
