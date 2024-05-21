#[macro_use] extern crate rocket;

pub mod api;
pub mod models;
pub mod serpapi_client;
pub mod openai_client;
pub mod config;

pub use api::health_check;
