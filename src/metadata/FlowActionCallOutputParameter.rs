use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowActionCallOutputParameter  {
	#[serde(rename = "assignToReference")]
	pub assign_to_reference: String,
	#[serde(rename = "name")]
	pub name: String,
}
