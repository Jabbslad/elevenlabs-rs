use serde::Deserialize;

use crate::error::Result;
use crate::ElevenLabsClient;

#[derive(Deserialize, Debug, Clone)]
pub struct Voice {
    pub voice_id: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub preview_url: String,
    pub available_for_tiers: Vec<String>,
    pub high_quality_base_model_ids: Vec<String>,
    pub owner_id: Option<String>,
    pub permission_on_resource: Option<String>,
    pub is_legacy: bool,
}

impl ElevenLabsClient {
    pub async fn voices(&self) -> Result<Vec<Voice>> {
        let path = "voices";
        let response = self.get(path).await?;
        let value: serde_json::Value = response.json().await?;
        let voices: Vec<Voice> = serde_json::from_value(value["voices"].clone())?;
        Ok(voices)
    }
}
