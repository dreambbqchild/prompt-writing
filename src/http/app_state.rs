use std::path::Path;
use crate::data::writing_prompt_repository::WritingPromptRepository;

#[derive(Clone)]
pub struct AppState { 
    pub db: WritingPromptRepository
}

impl AppState {
    pub fn new<P: AsRef<Path>>(path: P) -> AppState {
        AppState {
            db: WritingPromptRepository::new(path)
        }
    }
}