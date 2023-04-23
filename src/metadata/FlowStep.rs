use crate::metadata::FlowConnector::FlowConnector;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowStep  {
	#[serde(rename = "connectors")]
	pub connectors: Option<Vec<FlowConnector>>,
}
