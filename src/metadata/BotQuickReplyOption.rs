use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotQuickReplyOption  {
	#[serde(rename = "literalValue")]
	pub literal_value: String,
	#[serde(rename = "quickReplyOptionIdentifier")]
	pub quick_reply_option_identifier: Option<String>,
}
