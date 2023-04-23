use crate::metadata::ConversationVariableType::ConversationVariableType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotNavigationLink  {
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "targetBotDialog")]
	pub target_bot_dialog: Option<String>,
	#[serde(rename = "targetVariable")]
	pub target_variable: Option<String>,
	#[serde(rename = "targetVariableType")]
	pub target_variable_type: Option<ConversationVariableType>,
}
