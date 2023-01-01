use cachier::cachier::Cachier;


fn main() {
    let cachier = Cachier::new("https://example.com/api".to_string(), "memory".to_string());
    match cachier.get(Some("key")) {
        Ok(value) => println!("Received value: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
