use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowOutputFieldAssignment  {
	#[serde(rename = "assignToReference")]
	pub assign_to_reference: String,
	#[serde(rename = "field")]
	pub field: String,
}
