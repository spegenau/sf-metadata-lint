use crate::metadata::ConversationRecordLookupCondition::ConversationRecordLookupCondition;
use crate::metadata::ConversationRecordLookupField::ConversationRecordLookupField;
use crate::metadata::ConversationVariableType::ConversationVariableType;
use crate::metadata::SortOrder::SortOrder;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConversationRecordLookup  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: String,
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<ConversationRecordLookupCondition>>,
	#[serde(rename = "filterLogic")]
	pub filter_logic: Option<String>,
	#[serde(rename = "lookupFields")]
	pub lookup_fields: Option<Vec<ConversationRecordLookupField>>,
	#[serde(rename = "maxLookupResults")]
	pub max_lookup_results: i32,
	#[serde(rename = "sortFieldName")]
	pub sort_field_name: Option<String>,
	#[serde(rename = "sortOrder")]
	pub sort_order: Option<SortOrder>,
	#[serde(rename = "sourceVariableName")]
	pub source_variable_name: Option<String>,
	#[serde(rename = "sourceVariableType")]
	pub source_variable_type: Option<ConversationVariableType>,
	#[serde(rename = "targetVariableName")]
	pub target_variable_name: String,
}
