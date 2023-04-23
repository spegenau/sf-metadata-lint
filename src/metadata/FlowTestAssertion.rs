use crate::metadata::FlowTestCondition::FlowTestCondition;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTestAssertion  {
	#[serde(rename = "conditions")]
	pub conditions: Option<Vec<FlowTestCondition>>,
	#[serde(rename = "errorMessage")]
	pub error_message: Option<String>,
}
