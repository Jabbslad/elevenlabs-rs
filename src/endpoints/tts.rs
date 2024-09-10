use crate::error::Result;
use serde::Serialize;

use crate::ElevenLabsClient;

#[derive(Serialize)]
pub struct TextToSpeech {
    text: String,
}

impl ElevenLabsClient {
    pub async fn tts(&self, voice_id: &str, tts: &TextToSpeech) -> Result<Vec<u8>> {
        let path = format!("text-to-speech/{voice_id}");
        let response = self.post(&path, tts).await?;
        Ok(response.bytes().await?.to_vec())
    }
}
