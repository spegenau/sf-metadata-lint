use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowRule::FlowRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowDecision  {
	#[serde(rename = "defaultConnector")]
	pub default_connector: Option<FlowConnector>,
	#[serde(rename = "defaultConnectorLabel")]
	pub default_connector_label: Option<String>,
	#[serde(rename = "rules")]
	pub rules: Option<Vec<FlowRule>>,
}
