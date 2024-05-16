#[macro_use] extern crate rocket;

mod api;
mod models;
mod serpapi_client;
mod openai_client;
mod config;

use config::init;

#[launch]
fn rocket() -> _ {
    init();
    rocket::build().mount("/", routes![api::chat_completions])
}
