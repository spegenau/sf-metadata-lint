use crate::metadata::FlowAssignmentOperator::FlowAssignmentOperator;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCollectionMapItem  {
	#[serde(rename = "assignToFieldReference")]
	pub assign_to_field_reference: String,
	#[serde(rename = "operator")]
	pub operator: FlowAssignmentOperator,
	#[serde(rename = "value")]
	pub value: FlowElementReferenceOrValue,
}
