use crate::metadata::ApprovalAction::ApprovalAction;
use crate::metadata::ApprovalEntryCriteria::ApprovalEntryCriteria;
use crate::metadata::ApprovalStepApprover::ApprovalStepApprover;
use crate::metadata::ApprovalStepRejectBehavior::ApprovalStepRejectBehavior;
use crate::metadata::StepCriteriaNotMetType::StepCriteriaNotMetType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalStep  {
	#[serde(rename = "allowDelegate")]
	pub allow_delegate: Option<bool>,
	#[serde(rename = "approvalActions")]
	pub approval_actions: Option<ApprovalAction>,
	#[serde(rename = "assignedApprover")]
	pub assigned_approver: ApprovalStepApprover,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "entryCriteria")]
	pub entry_criteria: Option<ApprovalEntryCriteria>,
	#[serde(rename = "ifCriteriaNotMet")]
	pub if_criteria_not_met: Option<StepCriteriaNotMetType>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "rejectBehavior")]
	pub reject_behavior: Option<ApprovalStepRejectBehavior>,
	#[serde(rename = "rejectionActions")]
	pub rejection_actions: Option<ApprovalAction>,
}
