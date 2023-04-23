use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotMessage  {
	#[serde(rename = "message")]
	pub message: String,
	#[serde(rename = "messageIdentifier")]
	pub message_identifier: Option<String>,
}
