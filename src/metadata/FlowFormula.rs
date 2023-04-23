use crate::metadata::FlowDataType::FlowDataType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowFormula  {
	#[serde(rename = "dataType")]
	pub data_type: Option<FlowDataType>,
	#[serde(rename = "expression")]
	pub expression: String,
	#[serde(rename = "scale")]
	pub scale: Option<i32>,
}
