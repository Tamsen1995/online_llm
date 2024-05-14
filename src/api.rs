use rocket::serde::{json::Json};
use rocket::response::status;
use rocket::http::Status;
use std::error::Error;
use crate::models::{ChatRequest, ChatResponse};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest, Content, MessageRole};
use std::env;

#[post("/chat/completions", format = "json", data = "<chat_request>")]
pub async fn chat_completions(chat_request: Json<ChatRequest>) -> Result<Json<ChatResponse>, status::Custom<String>> {
    // Call OpenAI API for chat completion
    let openai_response = match call_openai_api(&chat_request).await {
        Ok(response) => response,
        Err(e) => return Err(status::Custom(Status::InternalServerError, e.to_string())),
    };
    
    // Combine the OpenAI response
    let combined_response = format!("OpenAI response: {}", openai_response);

    Ok(Json(ChatResponse { response: combined_response }))
}

async fn call_openai_api(chat_request: &ChatRequest) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new(api_key);

    let req = ChatCompletionRequest::new(
        "gpt-3.5-turbo".to_string(),
        vec![
            chat_completion::ChatCompletionMessage {
                role: MessageRole::system,
                // TODO: Change this to be more dynamic
                content: Content::Text("You are a helpful assistant!".to_string()),
                name: None,
            },
            chat_completion::ChatCompletionMessage {
                role: MessageRole::user,
                content: Content::Text(chat_request.messages[0].content.clone()),
                name: None,
            },
        ],
    );

    let result = client.chat_completion(req)?;
    Ok(result.choices[0].message.content.clone().unwrap_or_default())
}
