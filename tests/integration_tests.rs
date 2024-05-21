use rocket::local::asynchronous::Client;
use rocket::http::Status;
use rocket::routes;
use online_llm::api::{health_check, index, chat_completions};
use serde_json::json;

fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![health_check, index, chat_completions])
}

#[tokio::test]
async fn test_health_check() {
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let response = client.get("/health").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.unwrap();
    assert_eq!(body, "OK");
}

#[tokio::test]
async fn test_index() {
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");
    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.unwrap();
    assert_eq!(body, "This online llm works and returns responses!");
}
