use crate::metadata::WorkflowActionReference::WorkflowActionReference;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalAction  {
	#[serde(rename = "action")]
	pub action: Option<Vec<WorkflowActionReference>>,
}
