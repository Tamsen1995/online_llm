use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, ChatCompletionMessage, ChatCompletionResponse, Content, MessageRole};
use crate::config;
use crate::models::ChatRequest;
use std::error::Error;

pub async fn call_openai_api(chat_request: &ChatRequest, web_data: &str) -> Result<ChatCompletionResponse, Box<dyn Error>> {
    let api_key = config::openai_api_key();
    let client = Client::new(api_key);

    let req = ChatCompletionRequest::new(
        chat_request.model.clone(),
        vec![
            ChatCompletionMessage {
                role: MessageRole::system,
                content: Content::Text(chat_request.messages[0].content.clone()),
                name: None,
            },
            ChatCompletionMessage {
                role: MessageRole::user,
                content: Content::Text(chat_request.messages[1].content.clone()),
                name: None,
            },
            ChatCompletionMessage {
                role: MessageRole::system,
                content: Content::Text(format!("Relevant information: {}", web_data)),
                name: None,
            },
        ],
    );

    let result = client.chat_completion(req);

    match result {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("Error calling OpenAI API: {:?}", e);
            Err(Box::new(e) as Box<dyn Error>)
        }
    }
}
