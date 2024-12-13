use serde::Deserialize;

#[derive(Deserialize)]
pub struct SessionStartForm {
    pub seconds: u64
}

#[derive(Deserialize)]
pub struct AddPromptForm {
    pub text: String
}