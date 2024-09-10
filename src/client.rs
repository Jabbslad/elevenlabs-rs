use reqwest::Method;
use serde::Serialize;
use crate::error::Result;

pub struct ElevenLabsClient {
    pub client: reqwest::Client,
    pub api_key: String,
}

const API_BASE_URL: &str = "https://api.elevenlabs.io/v1";

impl ElevenLabsClient {
    /// Creates a new [`ElevenLabsClient`].
    pub fn new(api_key: &str) -> Self {
        let client = reqwest::Client::new();
        ElevenLabsClient {
            client,
            api_key: api_key.to_string(),
        }
    }

    fn builder(&self, method: Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}/{}", API_BASE_URL, path);
        self.client
            .request(method, url)
            .header("xi-api-key", &self.api_key)
    }

    pub async fn get(&self, path: &str) -> Result<reqwest::Response> {
        let response = self.builder(Method::GET, &path).send().await?;
        Ok(response)
    }

    pub async fn post<T: Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<reqwest::Response> {
        let response = self.builder(Method::POST, &path).json(body).send().await?;
        Ok(response)
    }
}
