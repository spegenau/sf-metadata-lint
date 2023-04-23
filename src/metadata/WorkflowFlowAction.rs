use crate::metadata::WorkflowFlowActionParameter::WorkflowFlowActionParameter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowFlowAction  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "flow")]
	pub flow: String,
	#[serde(rename = "flowInputs")]
	pub flow_inputs: Option<Vec<WorkflowFlowActionParameter>>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "language")]
	pub language: Option<String>,
	#[serde(rename = "protected")]
	pub protected: bool,
}
