use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use crate::metadata::FlowRecordFilterOperator::FlowRecordFilterOperator;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowRecordFilter  {
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "operator")]
	pub operator: FlowRecordFilterOperator,
	#[serde(rename = "value")]
	pub value: Option<FlowElementReferenceOrValue>,
}
