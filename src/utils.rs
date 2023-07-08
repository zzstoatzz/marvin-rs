use crate::models::{Message, ChatResponse};
use serde_json::json;
use reqwest::{Client};
use std::error::Error;

pub async fn get_chat_completion(
    messages: Vec<Message>
) -> Result<ChatResponse, Box<dyn Error>> {
    let request_url = "https://api.openai.com/v1/chat/completions";
    let api_key = std::env::var("OPENAI_API_KEY")?;

    let client = Client::new();

    let res = client.post(request_url)
        .bearer_auth(api_key)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": messages
        }))
        .send()
        .await?;

    let res_body: ChatResponse = res.json().await?;
    Ok(res_body)
}

