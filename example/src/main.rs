use reqwest::Client;
use serde_json::json;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let cachier = Cachier::new("https://example.com/api".to_string(), "memory".to_string());

    // Call the `set` method to store a value in the cache
    let is_saved = cachier.set("key", json!("value"), Some(3600)).await;
    println!("Is saved successfully: {}", is_saved);

    // Call the `get` method to retrieve the value from the cache
    let result = cachier.get(Some("key")).await;
    println!("Result: {:?}", result);
}
