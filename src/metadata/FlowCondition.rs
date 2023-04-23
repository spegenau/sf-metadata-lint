use crate::metadata::FlowComparisonOperator::FlowComparisonOperator;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCondition  {
	#[serde(rename = "leftValueReference")]
	pub left_value_reference: String,
	#[serde(rename = "operator")]
	pub operator: FlowComparisonOperator,
	#[serde(rename = "rightValue")]
	pub right_value: Option<FlowElementReferenceOrValue>,
}
