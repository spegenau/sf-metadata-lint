use crate::metadata::ConversationDataType::ConversationDataType;
use crate::metadata::ConversationVariableCollectionType::ConversationVariableCollectionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationVariable  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: Option<String>,
	#[serde(rename = "collectionType")]
	pub collection_type: Option<ConversationVariableCollectionType>,
	#[serde(rename = "dataType")]
	pub data_type: ConversationDataType,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "label")]
	pub label: String,
}
