use crate::metadata::FlowAssignmentItem::FlowAssignmentItem;
use crate::metadata::FlowConnector::FlowConnector;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowAssignment  {
	#[serde(rename = "assignmentItems")]
	pub assignment_items: Option<Vec<FlowAssignmentItem>>,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
}
