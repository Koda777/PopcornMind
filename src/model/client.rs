use crate::model::chat_gpt::ModelClientError;
use std::future::Future;
use std::pin::Pin;
use crate::model::api_response::ApiResponse;

pub trait ModelClient {
    fn send_message<'a>(&'a self, text: &'a str) -> Pin<Box<dyn Future<Output = Result<ApiResponse, ModelClientError>> + Send + 'a>>;
}
