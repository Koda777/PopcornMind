
use serde::Deserialize;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub output: Vec<OutputMessage>,
    pub usage: Usage,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct OutputMessage {
    pub content: Vec<ContentItem>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ContentItem {
    pub text: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub total_tokens: u32,
}

impl fmt::Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(text) = self.output.get(0)
            .and_then(|m| m.content.get(0))
            .map(|c| &c.text)
        {
            write!(f, "{}", text)
        } else {
            write!(f, "No output available.")
        }
    }
}
