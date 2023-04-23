use crate::metadata::FlowTestParameterType::FlowTestParameterType;
use crate::metadata::FlowTestReferenceOrValue::FlowTestReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTestParameter  {
	#[serde(rename = "leftValueReference")]
	pub left_value_reference: String,
	#[serde(rename = "type")]
	pub _type: FlowTestParameterType,
	#[serde(rename = "value")]
	pub value: FlowTestReferenceOrValue,
}
