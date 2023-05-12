use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EinsteinAssistantSettings  {
	#[serde(rename = "enableEinsteinAssistantDataExtractionEnabled")]
	pub enable_einstein_assistant_data_extraction_enabled: Option<bool>,
	#[serde(rename = "enableEinsteinAssistantEnabled")]
	pub enable_einstein_assistant_enabled: Option<bool>,
	#[serde(rename = "enableEinsteinEnableVoiceLogging")]
	pub enable_einstein_enable_voice_logging: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
