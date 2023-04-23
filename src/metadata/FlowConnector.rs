use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowConnector  {
	#[serde(rename = "isGoTo")]
	pub is_go_to: Option<bool>,
	#[serde(rename = "targetReference")]
	pub target_reference: String,
}
