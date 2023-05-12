use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use crate::metadata::FlowTransformValueActionType::FlowTransformValueActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTransformValueAction  {
	#[serde(rename = "outputFieldApiName")]
	pub output_field_api_name: Option<String>,
	#[serde(rename = "transformType")]
	pub transform_type: FlowTransformValueActionType,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
