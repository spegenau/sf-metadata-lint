use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotMessageTranslation  {
	#[serde(rename = "message")]
	pub message: Option<String>,
	#[serde(rename = "messageIdentifier")]
	pub message_identifier: String,
}
