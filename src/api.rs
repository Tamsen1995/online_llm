use rocket::serde::{json::Json};
use rocket::response::status;
use rocket::http::Status;
use crate::models::{ChatRequest, ChatResponse};
use crate::serpapi_client;
use crate::openai_client;

#[post("/chat/completions", format = "json", data = "<chat_request>")]
pub async fn chat_completions(chat_request: Json<ChatRequest>) -> Result<Json<ChatResponse>, status::Custom<String>> {
    let web_data = match serpapi_client::search(&chat_request.messages[0].content).await {
        Ok(data) => data,
        Err(e) => return Err(status::Custom(Status::InternalServerError, e.to_string())),
    };

    let openai_response = match openai_client::call_openai_api(&chat_request, &web_data).await {
        Ok(response) => response,
        Err(e) => return Err(status::Custom(Status::InternalServerError, e.to_string())),
    };

    Ok(Json(ChatResponse { response: openai_response }))
}
