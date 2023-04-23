use crate::metadata::ConversationSystemMessageMapping::ConversationSystemMessageMapping;
use crate::metadata::ConversationSystemMessageType::ConversationSystemMessageType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationSystemMessage  {
	#[serde(rename = "systemMessageMappings")]
	pub system_message_mappings: Option<Vec<ConversationSystemMessageMapping>>,
	#[serde(rename = "type")]
	pub _type: ConversationSystemMessageType,
}
