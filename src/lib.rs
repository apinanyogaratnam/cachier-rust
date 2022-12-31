use std::time::Duration;
use std::error::Error;

pub struct Cachier {
    url: String,
    storage: String,
}

impl Cachier {
    pub fn new(url: String, storage: String) -> Self {
        return Cachier {url: url, storage: storage};
    }

    pub async fn get(&self, key: Option<&str>) {
        match key {
            Some(key) => {
                let result = self.__get(key).await;
                match result {
                    Ok(result) => println!("{:?}", result),
                    Err(result) => println!("{:?}", result),
                }
            },
            None => println!("none"),
        }
    }

    async fn __get(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();
        let doge = client
        .get(format!("{url}?cache_key={key}&driver={driver}", url=self.url, key=key, driver=self.storage))
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
        println!("{:}", doge);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
