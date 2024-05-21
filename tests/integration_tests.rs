use rocket::local::asynchronous::Client;
use rocket::http::Status;
use rocket::routes;
use online_llm::health_check; // Import from lib.rs

// Include the Rocket instance for testing purposes
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", routes![health_check])
}

// Test for the /health endpoint
#[tokio::test]
async fn test_health_check() {
    // Create a client to issue requests to the Rocket instance
    let client = Client::tracked(rocket()).await.expect("valid rocket instance");

    // Issue a GET request to the /health endpoint
    let response = client.get("/health").dispatch().await;

    // Assert that the response status is OK (200)
    assert_eq!(response.status(), Status::Ok);

    // Assert that the response body is "OK"
    let body = response.into_string().await.unwrap();
    assert_eq!(body, "OK");
}
