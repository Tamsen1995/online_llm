use rocket::serde::{json::Json};
use rocket::response::status;
use rocket::http::Status;
use crate::models::ChatRequest;
use crate::serpapi_client;
use crate::openai_client;

fn extract_keywords(query: &str) -> Option<String> {
    let stop_words = vec!["a", "an", "the", "in", "on", "at", "with", "and", "but", "or"];
    let words: Vec<&str> = query
        .split_whitespace()
        .filter(|word| !stop_words.contains(&word.to_lowercase().as_str()))
        .collect();
    
    if words.is_empty() {
        None
    } else {
        Some(words.join(" "))
    }
}

#[get("/")]
pub fn index() -> &'static str {
    "This online llm works and returns responses!"
}

#[get("/health")]
pub fn health_check() -> &'static str {
    println!("OK");
    "OK"
}

#[post("/chat/completions", format = "json", data = "<chat_request>")]
pub async fn chat_completions(chat_request: Json<ChatRequest>) -> Result<Json<serde_json::Value>, status::Custom<String>> {
    let user_query = &chat_request.messages[1].content;

    let query = match extract_keywords(user_query) {
        Some(q) => q,
        None => return Err(status::Custom(Status::BadRequest, "Invalid query.".to_string())),
    };

    let web_data = match serpapi_client::search(&query).await {
        Ok(data) => data,
        Err(e) => return Err(status::Custom(Status::InternalServerError, format!("Failed to fetch web data: {}", e))),
    };

    let openai_response = match openai_client::call_openai_api(&chat_request, &web_data).await {
        Ok(response) => response,
        Err(e) => return Err(status::Custom(Status::InternalServerError, format!("Error calling OpenAI API: {}", e))),
    };

    // Return the raw OpenAI response
    Ok(Json(serde_json::to_value(openai_response).map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))?))
}
