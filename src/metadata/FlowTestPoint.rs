use crate::metadata::FlowTestAssertion::FlowTestAssertion;
use crate::metadata::FlowTestParameter::FlowTestParameter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTestPoint  {
	#[serde(rename = "assertions")]
	pub assertions: Option<Vec<FlowTestAssertion>>,
	#[serde(rename = "elementApiName")]
	pub element_api_name: String,
	#[serde(rename = "parameters")]
	pub parameters: Option<Vec<FlowTestParameter>>,
}
