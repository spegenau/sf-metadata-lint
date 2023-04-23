use crate::metadata::FieldMappingRow::FieldMappingRow;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FieldMapping  {
	#[serde(rename = "SObjectType")]
	pub s_object_type: String,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "fieldMappingRows")]
	pub field_mapping_rows: Option<Vec<FieldMappingRow>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
