use tide::http::{headers, StatusCode};
use tide::{Request, Response, Result};

use crate::runner;
use crate::State;

pub async fn index(_: Request<State>) -> Result<Response> {
    let index_html: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/index.html"));
    let response = Response::new(StatusCode::Ok)
        .body(index_html.as_bytes())
        .set_header(headers::CONTENT_TYPE, "text/html");

    Ok(response)
}

pub async fn run_test(req: Request<State>) -> Result<Response> {
    let (_, runner) = req.state();

    // Try running a test in the background.
    // If a test is already running, return an error response.
    let response = if runner::try_run(runner).await {
        Response::new(StatusCode::Ok).body_string("Ok".to_owned())
    } else {
        Response::new(StatusCode::Conflict)
            .body("A test is already running.".as_bytes())
            .set_header(headers::CONTENT_TYPE, "text/plain")
    };

    Ok(response)
}

pub async fn get_results(req: Request<State>) -> Result<Response> {
    let (db, _) = req.state();
    let results = db.get_all_results().await.unwrap();
    let response = Response::new(StatusCode::Ok).body_json(&results)?;
    Ok(response)
}
