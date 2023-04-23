use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowWaitEventOutputParameter  {
	#[serde(rename = "assignToReference")]
	pub assign_to_reference: String,
	#[serde(rename = "name")]
	pub name: String,
}
