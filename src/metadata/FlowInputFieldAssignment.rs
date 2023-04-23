use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowInputFieldAssignment  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
