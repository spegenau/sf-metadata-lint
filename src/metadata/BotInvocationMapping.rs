use crate::metadata::BotInvocationMappingType::BotInvocationMappingType;
use crate::metadata::ConversationVariableType::ConversationVariableType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BotInvocationMapping  {
	#[serde(rename = "parameterName")]
	pub parameter_name: String,
	#[serde(rename = "recordName")]
	pub record_name: Option<String>,
	#[serde(rename = "type")]
	pub _type: BotInvocationMappingType,
	#[serde(rename = "value")]
	pub value: Option<String>,
	#[serde(rename = "variableName")]
	pub variable_name: Option<String>,
	#[serde(rename = "variableType")]
	pub variable_type: Option<ConversationVariableType>,
}
