use crate::metadata::ConversationContextVariableMapping::ConversationContextVariableMapping;
use crate::metadata::ConversationDataType::ConversationDataType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationContextVariable  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: Option<String>,
	#[serde(rename = "contextVariableMappings")]
	pub context_variable_mappings: Option<Vec<ConversationContextVariableMapping>>,
	#[serde(rename = "dataType")]
	pub data_type: ConversationDataType,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: String,
}
