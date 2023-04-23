use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreenRuleAction  {
	#[serde(rename = "attribute")]
	pub attribute: String,
	#[serde(rename = "fieldReference")]
	pub field_reference: String,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
