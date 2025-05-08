mod model;

use tokio::main;
use crate::model::chat_gpt::{ChatGpt, ModelClientError};
use crate::model::chat_gpt::ModelType::GPT4_1;
use crate::model::client::ModelClient;

#[main]
async fn main() -> Result<(), ModelClientError> {
    dotenv::dotenv().ok();
    let client = ChatGpt::new(GPT4_1).expect("Unable to create client");
    let response = client.send_message("Hello").await?;
    println!("Response: {}", response);
    Ok(())
}