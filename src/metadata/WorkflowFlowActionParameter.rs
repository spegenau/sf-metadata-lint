use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowFlowActionParameter  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: Option<String>,
}
