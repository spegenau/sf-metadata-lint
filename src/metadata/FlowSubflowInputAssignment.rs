use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowSubflowInputAssignment  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
