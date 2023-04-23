use crate::metadata::MLFieldType::MLFieldType;
use crate::metadata::MLRelationType::MLRelationType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MLField  {
	#[serde(rename = "entity")]
	pub entity: Option<String>,
	#[serde(rename = "entityName")]
	pub entity_name: Option<String>,
	#[serde(rename = "field")]
	pub field: Option<String>,
	#[serde(rename = "fieldName")]
	pub field_name: Option<String>,
	#[serde(rename = "relatedField")]
	pub related_field: Option<Box<MLField>>,
	#[serde(rename = "relationType")]
	pub relation_type: Option<MLRelationType>,
	#[serde(rename = "type")]
	pub _type: MLFieldType,
}
