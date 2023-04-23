use crate::metadata::WorkflowActionType::WorkflowActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkflowActionReference  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "type")]
	pub _type: WorkflowActionType,
}
