use crate::metadata::ServiceAISetupFieldType::ServiceAISetupFieldType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServiceAISetupField  {
	#[serde(rename = "entity")]
	pub entity: String,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "fieldMappingType")]
	pub field_mapping_type: ServiceAISetupFieldType,
	#[serde(rename = "fieldPosition")]
	pub field_position: i32,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "setupDefinition")]
	pub setup_definition: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
