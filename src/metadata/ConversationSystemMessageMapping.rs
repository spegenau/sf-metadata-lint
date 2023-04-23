use crate::metadata::ConversationMappingType::ConversationMappingType;
use crate::metadata::ConversationSystemMessageParamType::ConversationSystemMessageParamType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationSystemMessageMapping  {
	#[serde(rename = "mappingType")]
	pub mapping_type: ConversationMappingType,
	#[serde(rename = "parameterType")]
	pub parameter_type: ConversationSystemMessageParamType,
	#[serde(rename = "variableName")]
	pub variable_name: String,
}
