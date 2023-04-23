use crate::metadata::BotMessageTranslation::BotMessageTranslation;
use crate::metadata::BotQuickReplyOptionTranslation::BotQuickReplyOptionTranslation;
use crate::metadata::BotVariableOperationType::BotVariableOperationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotVariableOperationTranslation  {
	#[serde(rename = "botMessages")]
	pub bot_messages: Option<Vec<BotMessageTranslation>>,
	#[serde(rename = "botQuickReplyOptions")]
	pub bot_quick_reply_options: Option<Vec<BotQuickReplyOptionTranslation>>,
	#[serde(rename = "quickReplyOptionTemplate")]
	pub quick_reply_option_template: Option<String>,
	#[serde(rename = "retryMessages")]
	pub retry_messages: Option<Vec<BotMessageTranslation>>,
	#[serde(rename = "successMessages")]
	pub success_messages: Option<Vec<BotMessageTranslation>>,
	#[serde(rename = "type")]
	pub _type: BotVariableOperationType,
	#[serde(rename = "variableOperationIdentifier")]
	pub variable_operation_identifier: String,
}
