use crate::metadata::FlowProcessType::FlowProcessType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCoverageResult  {
	#[serde(rename = "elementsNotCovered")]
	pub elements_not_covered: Option<Vec<String>>,
	#[serde(rename = "flowId")]
	pub flow_id: String,
	#[serde(rename = "flowName")]
	pub flow_name: String,
	#[serde(rename = "flowNamespace")]
	pub flow_namespace: String,
	#[serde(rename = "numElements")]
	pub num_elements: i32,
	#[serde(rename = "numElementsNotCovered")]
	pub num_elements_not_covered: i32,
	#[serde(rename = "processType")]
	pub process_type: FlowProcessType,
}
