use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomShortcut  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "eventName")]
	pub event_name: String,
	#[serde(rename = "action")]
	pub action: String,
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "keyCommand")]
	pub key_command: String,
}
