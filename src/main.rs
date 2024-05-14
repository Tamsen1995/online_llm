#[macro_use] extern crate rocket;

mod api;
mod models;

use dotenv::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![api::chat_completions])
}
