use crate::metadata::ObjectRelationshipType::ObjectRelationshipType;
use crate::metadata::UIObjectRelationFieldConfig::UIObjectRelationFieldConfig;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UIObjectRelationConfig  {
	#[serde(rename = "UIObjectRelationFieldConfigs")]
	pub ui_object_relation_field_configs: Option<Vec<UIObjectRelationFieldConfig>>,
	#[serde(rename = "contextObject")]
	pub context_object: String,
	#[serde(rename = "contextObjectRecordType")]
	pub context_object_record_type: Option<String>,
	#[serde(rename = "directRelationshipField")]
	pub direct_relationship_field: Option<String>,
	#[serde(rename = "indirectObjectContextField")]
	pub indirect_object_context_field: Option<String>,
	#[serde(rename = "indirectObjectRelatedField")]
	pub indirect_object_related_field: Option<String>,
	#[serde(rename = "indirectRelationshipObject")]
	pub indirect_relationship_object: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "relatedObject")]
	pub related_object: String,
	#[serde(rename = "relatedObjectRecordType")]
	pub related_object_record_type: Option<String>,
	#[serde(rename = "relationshipType")]
	pub relationship_type: ObjectRelationshipType,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
