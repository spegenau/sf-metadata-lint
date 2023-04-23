use crate::metadata::StepRejectBehaviorType::StepRejectBehaviorType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalStepRejectBehavior  {
	#[serde(rename = "type")]
	pub _type: StepRejectBehaviorType,
}
