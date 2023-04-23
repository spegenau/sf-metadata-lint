use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowConstant  {
	#[serde(rename = "dataType")]
	pub data_type: FlowDataType,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
