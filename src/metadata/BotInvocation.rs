use crate::metadata::BotInvocationMapping::BotInvocationMapping;
use crate::metadata::ConversationInvocableTargetType::ConversationInvocableTargetType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotInvocation  {
	#[serde(rename = "invocationActionName")]
	pub invocation_action_name: Option<String>,
	#[serde(rename = "invocationActionType")]
	pub invocation_action_type: Option<ConversationInvocableTargetType>,
	#[serde(rename = "invocationMappings")]
	pub invocation_mappings: Option<Vec<BotInvocationMapping>>,
}
