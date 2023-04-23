use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowOutboundMessage  {
	#[serde(rename = "apiVersion")]
	pub api_version: f32,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "endpointUrl")]
	pub endpoint_url: String,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "includeSessionId")]
	pub include_session_id: bool,
	#[serde(rename = "integrationUser")]
	pub integration_user: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "protected")]
	pub protected: bool,
	#[serde(rename = "useDeadLetterQueue")]
	pub use_dead_letter_queue: Option<bool>,
}
