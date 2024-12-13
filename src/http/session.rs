use axum::{
    extract::{Path, State}, http::StatusCode, response::{sse::{Event, KeepAlive, Sse}, IntoResponse}, Form
};
use chrono::Utc;

use core::time;
use std::{convert::Infallible, thread, time::Duration};
use tokio_stream::StreamExt as _ ;
use futures_util::stream::{self, Stream};

use super::{app_state::AppState, forms::SessionStartForm};

fn delay_to_next_full_second() {
    let ms: u64 = (Utc::now().timestamp_millis() % 1000).try_into().unwrap();
    let ms_to_next_full_second = time::Duration::from_millis(1000 - ms);
    thread::sleep(ms_to_next_full_second);
}

pub(crate) async fn countdown_handler(state: State<AppState>, Path(key): Path<String>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(move || {
            let end_at = state.db.extract_datetime(&key);
            let now = Utc::now();
            let to_go = (end_at - now).num_seconds().max(-1);

            delay_to_next_full_second();

            Event::default().data(format!("{to_go}"))
        })
        .map(Ok)
        .throttle(Duration::from_millis(750));

    delay_to_next_full_second();
    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub(crate) async fn current_prompt_handler(state: State<AppState>, Path(key): Path<String>) -> String {
    state.db.get_current_prompt_for_session(&key)
}

pub(crate) async fn timestamp_handler(state: State<AppState>, Path(key): Path<String>) -> String {
    state.db.extract_datetime(key).to_rfc3339()
}

pub(crate) async fn start_handler(state: State<AppState>, Path(key): Path<String>, Form(form): Form<SessionStartForm>) -> impl IntoResponse {
    match state.db.random_prompt_for_session(&key) {
        Ok(_) => (StatusCode::OK, state.db.set_timelimit(&key, form.seconds).to_rfc3339()),
        Err(value) => (StatusCode::PRECONDITION_FAILED, value.to_string())
    }
}