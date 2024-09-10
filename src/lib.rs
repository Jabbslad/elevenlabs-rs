pub mod client;
pub mod endpoints;
pub mod error;

pub use client::ElevenLabsClient;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = ElevenLabsClient::new("api_key");
        assert_eq!(client.api_key, "api_key");
    }
}
