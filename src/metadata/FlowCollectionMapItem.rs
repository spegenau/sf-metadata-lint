use crate::metadata::FlowAssignmentOperator::FlowAssignmentOperator;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCollectionMapItem  {
	#[serde(rename = "assignToFieldReference")]
	pub assign_to_field_reference: String,
	#[serde(rename = "operator")]
	pub operator: FlowAssignmentOperator,
	#[serde(rename = "value")]
	pub value: FlowElementReferenceOrValue,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
