#[macro_use] extern crate rocket;

mod api;
mod models;
mod serpapi_client;
mod openai_client;
mod config;

use config::init;

#[get("/health")]
fn health_check() -> &'static str {
    println!("OK");
    "OK"
}


#[launch]
fn rocket() -> _ {
    init();
    rocket::build()
        .mount("/", routes![api::index, api::chat_completions, health_check])
}
