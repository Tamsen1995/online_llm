use reqwest::Error;
use serde::Deserialize;
use serde_json;
use crate::config;

#[derive(Deserialize, Debug)]
struct SearchResult {
    organic_results: Option<Vec<ResultItem>>, // Updated to match actual field name
}

#[derive(Deserialize, Debug)]
struct ResultItem {
    title: String,
    snippet: String,
}

pub async fn search(query: &str) -> Result<String, Error> {
    let api_key = config::serpapi_api_key();
    let url = format!("https://serpapi.com/search.json?q={}&api_key={}", query, api_key);

    let response = reqwest::get(&url).await?;
    let text = response.text().await?;
    let result: Result<SearchResult, _> = serde_json::from_str(&text);

    match result {
        Ok(data) => {
            if let Some(results) = data.organic_results {
                let mut output = String::new();
                for item in results.iter().take(3) {
                    output.push_str(&format!("{}: {}\n", item.title, item.snippet));
                }
                println!("Relevant information found!!");
                Ok(output)
            } else {
                println!("No relevant information found!!");
                Ok("No relevant information found.".to_string())
            }
        }
        Err(e) => {
            eprintln!("Error deserializing SerpApi response: {}", e);
            Ok("Failed to parse SerpApi response.".to_string())
        }
    }
}
