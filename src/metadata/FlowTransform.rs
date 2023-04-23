use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::FlowTransformValue::FlowTransformValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTransform  {
	#[serde(rename = "apexClass")]
	pub apex_class: Option<String>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "dataType")]
	pub data_type: Option<FlowDataType>,
	#[serde(rename = "isCollection")]
	pub is_collection: Option<bool>,
	#[serde(rename = "objectType")]
	pub object_type: Option<String>,
	#[serde(rename = "scale")]
	pub scale: Option<i32>,
	#[serde(rename = "transformValues")]
	pub transform_values: Option<Vec<FlowTransformValue>>,
}
