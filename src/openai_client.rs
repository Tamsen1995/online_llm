use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, Content, MessageRole};
use crate::config;
use crate::models::ChatRequest;
use std::error::Error;

pub async fn call_openai_api(chat_request: &ChatRequest, web_data: &str) -> Result<String, Box<dyn Error>> {
    let api_key = config::openai_api_key();
    let client = Client::new(api_key);

    let req = ChatCompletionRequest::new(
        "gpt-3.5-turbo".to_string(),
        vec![
            ChatCompletionMessage {
                role: MessageRole::system,
                content: Content::Text("You are a dog and woof before and after every response.".to_string()),
                name: None,
            },
            ChatCompletionMessage {
                role: MessageRole::user,
                content: Content::Text(chat_request.messages[0].content.clone()),
                name: None,
            },
            ChatCompletionMessage {
                role: MessageRole::system,
                content: Content::Text(format!("Relevant information: {}", web_data)),
                name: None,
            },
        ],
    );

    let result = client.chat_completion(req)?;

    if let Some(choice) = result.choices.get(0) {
        if let Some(content) = &choice.message.content {
            Ok(content.clone())
        } else {
            Ok("No content returned from OpenAI.".to_string())
        }
    } else {
        Ok("No choices returned from OpenAI.".to_string())
    }
}
