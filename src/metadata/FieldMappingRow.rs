use crate::metadata::FieldMappingField::FieldMappingField;
use crate::metadata::MappingOperation::MappingOperation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldMappingRow  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: String,
	#[serde(rename = "fieldMappingFields")]
	pub field_mapping_fields: Option<Vec<FieldMappingField>>,
	#[serde(rename = "fieldName")]
	pub field_name: String,
	#[serde(rename = "mappingOperation")]
	pub mapping_operation: MappingOperation,
}
