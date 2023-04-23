use crate::metadata::FlowAssignmentOperator::FlowAssignmentOperator;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowAssignmentItem  {
	#[serde(rename = "assignToReference")]
	pub assign_to_reference: String,
	#[serde(rename = "operator")]
	pub operator: FlowAssignmentOperator,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
