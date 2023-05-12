use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowElement  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "name")]
	pub name: Option<String>,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
