use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DefaultShortcut  {
	#[serde(rename = "action")]
	pub action: String,
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "keyCommand")]
	pub key_command: String,
}
