use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowDataTypeMapping  {
	#[serde(rename = "typeName")]
	pub type_name: String,
	#[serde(rename = "typeValue")]
	pub type_value: String,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
