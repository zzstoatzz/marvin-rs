mod models;
mod utils;

use std::error::Error;
use crate::models::{Message, Role};
use crate::utils::get_chat_completion;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    // Create your messages here
    let messages = vec![
        Message {
            role: Role::System,
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: Role::User,
            content: "Who won the world series in 2020?".to_string(),
        },
    ];

    let chat_response = get_chat_completion(messages).await?;
    // Print out the first choice
    if let Some(first_choice) = chat_response.choices.first() {
        println!("First choice: {:?}", first_choice);
        println!("Tokens used: {}", chat_response.usage);
    } else {
        println!("No choices in response");
    }

    Ok(())
}

