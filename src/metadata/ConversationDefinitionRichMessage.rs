use crate::metadata::BotInvocationMapping::BotInvocationMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationDefinitionRichMessage  {
	#[serde(rename = "messageDefinitionMappings")]
	pub message_definition_mappings: Option<Vec<BotInvocationMapping>>,
	#[serde(rename = "messageDefinitionName")]
	pub message_definition_name: String,
}
