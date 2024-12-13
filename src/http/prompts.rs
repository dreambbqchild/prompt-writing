use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Form};

use super::{app_state::AppState, forms::AddPromptForm};

fn prompt_count_as_string(state: &AppState) -> String {
    let count = state.db.get_prompt_count();
    format!("{count}")
}

pub(crate) async fn add_handler(state: State<AppState>, Form(form): Form<AddPromptForm>) -> String {
    state.db.add_prompt(form.text);
    prompt_count_as_string(&state)
}

pub(crate) async fn count_handler(state: State<AppState>) -> String {
    prompt_count_as_string(&state)
}

pub(crate) async fn get_handler(state: State<AppState>, Path(key): Path<String>) -> impl IntoResponse {
    let value = state.db.get_prompt(&key);
    if value.len() == 0 {
        (StatusCode::PRECONDITION_FAILED, format!("Key {key} not found."))        
    } else {
        (StatusCode::OK, value)
    }
}