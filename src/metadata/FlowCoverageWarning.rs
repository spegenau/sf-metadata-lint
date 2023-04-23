use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowCoverageWarning  {
	#[serde(rename = "flowId")]
	pub flow_id: String,
	#[serde(rename = "flowName")]
	pub flow_name: String,
	#[serde(rename = "flowNamespace")]
	pub flow_namespace: String,
	#[serde(rename = "message")]
	pub message: String,
}
