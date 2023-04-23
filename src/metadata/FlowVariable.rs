use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowVariable  {
	#[serde(rename = "apexClass")]
	pub apex_class: Option<String>,
	#[serde(rename = "dataType")]
	pub data_type: FlowDataType,
	#[serde(rename = "isCollection")]
	pub is_collection: Option<bool>,
	#[serde(rename = "isInput")]
	pub is_input: Option<bool>,
	#[serde(rename = "isOutput")]
	pub is_output: Option<bool>,
	#[serde(rename = "objectType")]
	pub object_type: Option<String>,
	#[serde(rename = "scale")]
	pub scale: Option<i32>,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
