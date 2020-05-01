use async_std::task;
use tide::http::{headers, StatusCode};
use tide::{Request, Response, Result};

use crate::runner::Runner;
use crate::State;

pub async fn index(_: Request<State>) -> Result<Response> {
    let index_html: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/index.html"));
    let response = Response::new(StatusCode::Ok)
        .body(index_html.as_bytes())
        .set_header(headers::CONTENT_TYPE, "text/html");

    Ok(response)
}

pub async fn run_test(req: Request<State>) -> Result<Response> {
    let (db, runner) = req.state();
    let db = db.clone();

    // Try running a test. If a test is already running, return an error response.
    let future = match Runner::try_run(runner).await {
        None => {
            return Ok(Response::new(StatusCode::Conflict)
                .body_string("A test is already running.".to_owned()))
        }
        Some(future) => future,
    };

    // Run the test asynchronously, saving the results to the db on success.
    task::spawn(async move {
        if let Ok(result) = future.await {
            if let Err(e) = db.create_result(&result.into()).await {
                eprintln!("Database error: {}", e);
            }
        }
    });

    Ok(Response::new(StatusCode::Ok).body_string("Ok".to_owned()))
}

pub async fn get_results(req: Request<State>) -> Result<Response> {
    let (db, _) = req.state();
    let results = db.get_all_results().await.unwrap();
    let response = Response::new(StatusCode::Ok).body_json(&results)?;
    Ok(response)
}
