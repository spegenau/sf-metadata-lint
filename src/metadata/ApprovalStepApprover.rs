use crate::metadata::Approver::Approver;
use crate::metadata::RoutingType::RoutingType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalStepApprover  {
	#[serde(rename = "approver")]
	pub approver: Option<Vec<Approver>>,
	#[serde(rename = "whenMultipleApprovers")]
	pub when_multiple_approvers: Option<RoutingType>,
}
