use crate::metadata::ConversationVariableOperandSourceType::ConversationVariableOperandSourceType;
use crate::metadata::ConversationVariableType::ConversationVariableType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotVariableOperand  {
	#[serde(rename = "disableAutoFill")]
	pub disable_auto_fill: Option<bool>,
	#[serde(rename = "sourceName")]
	pub source_name: Option<String>,
	#[serde(rename = "sourceType")]
	pub source_type: Option<ConversationVariableOperandSourceType>,
	#[serde(rename = "sourceValue")]
	pub source_value: Option<String>,
	#[serde(rename = "targetName")]
	pub target_name: String,
	#[serde(rename = "targetType")]
	pub target_type: ConversationVariableType,
}
