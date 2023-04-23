use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowBaseElement  {
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
