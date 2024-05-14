use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
}

pub fn openai_api_key() -> String {
    env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set")
}

pub fn serpapi_api_key() -> String {
    env::var("SERPAPI_API_KEY").expect("SERPAPI_API_KEY must be set")
}
