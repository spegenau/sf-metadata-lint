use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceSetupAssistantSettings  {
	#[serde(rename = "enableServiceSetupAssistant")]
	pub enable_service_setup_assistant: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
