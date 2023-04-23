use crate::metadata::DefinitionCreationType::DefinitionCreationType;
use crate::metadata::InvalidMergeActionType::InvalidMergeActionType;
use crate::metadata::MktDataModelFieldUsageTag::MktDataModelFieldUsageTag;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MktDataModelFieldAttributes  {
	#[serde(rename = "definitionCreationType")]
	pub definition_creation_type: Option<DefinitionCreationType>,
	#[serde(rename = "invalidMergeActionType")]
	pub invalid_merge_action_type: Option<InvalidMergeActionType>,
	#[serde(rename = "isDynamicLookup")]
	pub is_dynamic_lookup: Option<bool>,
	#[serde(rename = "keyQualifierName")]
	pub key_qualifier_name: Option<String>,
	#[serde(rename = "primaryIndexOrder")]
	pub primary_index_order: Option<i32>,
	#[serde(rename = "refAttrDeveloperName")]
	pub ref_attr_developer_name: Option<String>,
	#[serde(rename = "usageTag")]
	pub usage_tag: Option<MktDataModelFieldUsageTag>,
}
