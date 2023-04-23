use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotQuickReplyOptionTranslation  {
	#[serde(rename = "literalValue")]
	pub literal_value: Option<String>,
	#[serde(rename = "quickReplyOptionIdentifier")]
	pub quick_reply_option_identifier: String,
}
