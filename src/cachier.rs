use std::time::Duration;
use std::error::Error;
use reqwest::{Client};
use serde_json::json;

pub struct Cachier {
    url: String,
    storage: String,
}

impl Cachier {
    #[allow(dead_code)]
    pub fn new(url: String, storage: String) -> Self {
        return Cachier{url: url, storage: storage};
    }

    /// Makes a GET request to the specified URL with the provided `key` and `driver` as query parameters.
    /// If `key` is `None`, it returns `"none"`. Otherwise, it returns the result of the GET request.
    ///
    /// # Examples
    ///
    /// ```
    /// let cachier = Cachier::new("https://example.com/api".to_string(), "memory".to_string());
    /// let result = cachier.get(Some("key")).await;
    /// println!("Result: {:?}", result);
    /// ```
    #[allow(dead_code)]
    pub async fn get(&self, key: Option<&str>) -> Result<String, Box<dyn Error>> {
        match key {
            Some(key) => self.__get(key).await,
            None => Err(From::from("key must be provided")),
        }
    }

    /// Sends a POST request to the specified URL with the provided `key`, `value`, `expiry`, and `driver`
    /// as a string in the request body. It then parses the JSON response and returns the value of the
    /// `is_saved_successfully` field as a boolean.
    ///
    /// # Examples
    ///
    /// ```
    /// let cachier = Cachier::new("https://example.com/api".to_string(), "memory".to_string());
    /// let is_saved = cachier.set("key", json!("value"), Some(3600)).await;
    /// println!("Is saved successfully: {}", is_saved);
    /// ```
    #[allow(dead_code)]
    pub async fn set(&self, key: &str, value: serde_json::Value, expiry: Option<i64>) -> bool {
        if key.is_empty() {
            return false;
        }

        let client = Client::new();
        let response = client
            .post(self.url.as_str())
            .body(json!({
                "cache_key": key,
                "cache_value": value,
                "cache_expiry": expiry,
                "driver": self.storage,
            }).to_string())
            .send()
            .await
            .unwrap();

        if response.status().as_u16() != 200 {
            return false;
        }

        let response_text = response.text().await.unwrap();
        let response_json: serde_json::Value = serde_json::from_str(&response_text).unwrap();

        response_json["is_saved_successfully"].as_bool().unwrap_or(false)
    }


    async fn __get(&self, key: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let value = client
            .get(format!("{url}?cache_key={key}&driver={driver}", url=self.url, key=key, driver=self.storage))
            .header("Accept", "text/plain")
            .timeout(Duration::from_secs(3))
            .send()
            .await?
            .text()
            .await?;

        Ok(value)
    }
}
