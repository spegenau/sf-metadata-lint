use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowStageStepEntryActionOutputParameter  {
	#[serde(rename = "assignToReference")]
	pub assign_to_reference: String,
	#[serde(rename = "name")]
	pub name: String,
}
