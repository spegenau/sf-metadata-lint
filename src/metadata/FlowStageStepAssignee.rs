use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use crate::metadata::FlowMetadataValue::FlowMetadataValue;
use crate::metadata::FlowStageStepAssigneeType::FlowStageStepAssigneeType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowStageStepAssignee  {
	#[serde(rename = "assignee")]
	pub assignee: FlowElementReferenceOrValue,
	#[serde(rename = "assigneeType")]
	pub assignee_type: FlowStageStepAssigneeType,
	#[serde(rename = "processMetadataValues")]
	pub process_metadata_values: Option<Vec<FlowMetadataValue>>,
}
