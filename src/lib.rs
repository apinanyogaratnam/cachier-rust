mod cachier;

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[tokio::test]
    async fn test_set_and_get() {
        let cache = cachier::Cachier::new("https://example.com/api".to_string(), "memory".to_string());

        // Test that the `set` method returns true when the value is stored successfully
        let is_saved = cache.set("key", json!("value"), Some(3600)).await;
        assert!(is_saved);

        // Test that the `get` method returns the correct value when the key is found in the cache
        let result = cache.get(Some("key")).await.unwrap();
        assert_eq!(result, "value");

        // Test that the `get` method returns "none" when the key is not found in the cache
        let result = cache.get(Some("invalid_key")).await.unwrap();
        assert_eq!(result, "none");
    }
}
