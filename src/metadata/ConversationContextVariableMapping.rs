use crate::metadata::MessageType::MessageType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationContextVariableMapping  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: String,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "messageType")]
	pub message_type: MessageType,
}
