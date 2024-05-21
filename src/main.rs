#[macro_use] extern crate rocket;

mod api;
mod models;
mod serpapi_client;
mod openai_client;
mod config;

use config::init;
use std::env;
use rocket::config::Config;
use online_llm::health_check; // Import from lib.rs

#[launch]
fn rocket() -> _ {
    // Initialize configuration
    init();

    // Read the PORT environment variable, defaulting to 8000 if not set
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 number");

    // Configure Rocket
    let config = Config {
        port,
        address: "0.0.0.0".parse().expect("Invalid address"),
        ..Config::default()
    };

    // Build and launch Rocket
    rocket::custom(config)
        .mount("/", routes![api::index, api::chat_completions, health_check])
}
