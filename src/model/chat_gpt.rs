use std::env::VarError;
use std::fmt;
use std::pin::Pin;
use reqwest::{Client};
use futures::FutureExt;
use reqwest::header::CONTENT_TYPE;
use serde_json::json;
use crate::model::api_response::ApiResponse;
use crate::model::client::{ModelClient};

#[derive(Debug)]
pub enum ModelClientError {
    Http(reqwest::Error),
    MissingApiKey(VarError),
    Parse(reqwest::Error),
}

impl fmt::Display for ModelClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelClientError::Http(e) => write!(f, "HTTP error: {}", e),
            ModelClientError::MissingApiKey(e) => write!(f, "Api key missing {}", e),
            ModelClientError::Parse(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ModelClientError {}

impl From<reqwest::Error> for ModelClientError {
    fn from(err: reqwest::Error) -> Self {
        ModelClientError::Http(err)
    }
}

pub enum ModelType {
    GPT4_1,
    O4Mini
}

impl ModelType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModelType::GPT4_1 => "gpt-4.1",
            ModelType::O4Mini => "o4-mini"
        }
    }
}

pub struct ChatGpt {
    api_key: String,
    endpoint: String,
    client: Client,
    model_type: ModelType,
}

impl ChatGpt {
    pub fn new(model_type: ModelType) -> Result<Self, ModelClientError> {
        let client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(5))
            .connection_verbose(true)
            .build().map_err(|e| ModelClientError::Http(e))?;
        Ok(ChatGpt {
            api_key: std::env::var("OPEN_API_KEY").map_err(|e| ModelClientError::MissingApiKey(e))?,
            endpoint: "https://api.openai.com/v1/responses".to_string(),
            client,
            model_type,
        })
    }

}

#[async_trait::async_trait]
impl ModelClient for ChatGpt {
    fn send_message<'a>(&'a self, text: &'a str) -> Pin<Box<dyn Future<Output = Result<ApiResponse, ModelClientError>> + Send + 'a>> {
        let req = self
            .client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .json(&json!({
                "model": self.model_type.as_str(),
                "input": text
            }));
        async move {
            let response = req
                .send()
                .await
                .map_err(|e| ModelClientError::Parse(e))?;
            let text = response
                .json::<ApiResponse>()
                .await
                .map_err(|e| ModelClientError::Parse(e))?;
            Ok(text)
        }.boxed()
    }
}
