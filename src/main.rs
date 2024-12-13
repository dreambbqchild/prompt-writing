mod http;
pub mod data;

use http::app_state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new("writing-prompts");
    http::start(state).await;
}