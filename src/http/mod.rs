pub mod app_state;
mod prompts;
mod session;
mod forms;

use std::env;

use app_state::AppState;
use axum::{
     routing::{get, put}, Router
};

use tower_http::services::ServeDir;

pub async fn start(state: AppState) {
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/prompts/add", put(prompts::add_handler))
        .route("/prompts/count", get(prompts::count_handler))
        .route("/prompts/get/:key", get(prompts::get_handler))
        .route("/session/:key/countdown", get(session::countdown_handler))
        .route("/session/:key/prompt", get(session::current_prompt_handler))
        .route("/session/:key/timestamp", get(session::timestamp_handler))
        .route("/session/:key/start", put(session::start_handler))
        .with_state(state);

    let addr = env::var("INET_ADDR").unwrap_or_else(|_| "0.0.0.0:3030".to_string());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Running at {addr}");
    axum::serve(listener, app).await.unwrap();
}