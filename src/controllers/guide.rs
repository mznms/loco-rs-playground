#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn echo(req_body: String) -> String {
    req_body
}

/// A function that returns a "hello" response.
///
/// # Arguments
///
/// * `State(_ctx)` - The application context.
///
/// # Returns
///
/// * A `Result` containing a `Response` with the text "hello" if successful.
///
/// # Errors
///
/// This function will return an error if there is an issue with creating the response.
#[debug_handler]
pub async fn hello(State(_ctx): State<AppContext>) -> Result<Response> {
    // do something with context (database, etc)
    format::text("hello")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("guide")
        .add("/", get(hello))
        .add("/echo", post(echo))
}
