use reqwest::Error;
use serde::Deserialize;
use crate::config;

#[derive(Deserialize)]
struct SearchResult {
    results: Vec<ResultItem>,  // Changed from `organic_results` to `results`
}

#[derive(Deserialize)]
struct ResultItem {
    title: String,
    snippet: String,
}

pub async fn search(query: &str) -> Result<String, Error> {
    let api_key = config::serpapi_api_key();
    let url = format!("https://serpapi.com/search.json?q={}&api_key={}", query, api_key);
    let response: SearchResult = reqwest::get(&url).await?.json().await?;

    let mut results = String::new();
    for item in response.results.iter().take(3) {
        results.push_str(&format!("{}: {}\n", item.title, item.snippet));
    }

    Ok(results)
}
