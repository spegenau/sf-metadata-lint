use crate::metadata::ApprovalAction::ApprovalAction;
use crate::metadata::ApprovalEntryCriteria::ApprovalEntryCriteria;
use crate::metadata::ApprovalPageField::ApprovalPageField;
use crate::metadata::ApprovalStep::ApprovalStep;
use crate::metadata::ApprovalSubmitter::ApprovalSubmitter;
use crate::metadata::NextAutomatedApprover::NextAutomatedApprover;
use crate::metadata::RecordEditabilityType::RecordEditabilityType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalProcess  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "allowRecall")]
	pub allow_recall: Option<bool>,
	#[serde(rename = "allowedSubmitters")]
	pub allowed_submitters: Option<Vec<ApprovalSubmitter>>,
	#[serde(rename = "approvalPageFields")]
	pub approval_page_fields: Option<ApprovalPageField>,
	#[serde(rename = "approvalStep")]
	pub approval_step: Option<Vec<ApprovalStep>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "emailTemplate")]
	pub email_template: Option<String>,
	#[serde(rename = "enableMobileDeviceAccess")]
	pub enable_mobile_device_access: Option<bool>,
	#[serde(rename = "entryCriteria")]
	pub entry_criteria: Option<ApprovalEntryCriteria>,
	#[serde(rename = "finalApprovalActions")]
	pub final_approval_actions: Option<ApprovalAction>,
	#[serde(rename = "finalApprovalRecordLock")]
	pub final_approval_record_lock: Option<bool>,
	#[serde(rename = "finalRejectionActions")]
	pub final_rejection_actions: Option<ApprovalAction>,
	#[serde(rename = "finalRejectionRecordLock")]
	pub final_rejection_record_lock: Option<bool>,
	#[serde(rename = "initialSubmissionActions")]
	pub initial_submission_actions: Option<ApprovalAction>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "nextAutomatedApprover")]
	pub next_automated_approver: Option<NextAutomatedApprover>,
	#[serde(rename = "postTemplate")]
	pub post_template: Option<String>,
	#[serde(rename = "processOrder")]
	pub process_order: Option<i32>,
	#[serde(rename = "recallActions")]
	pub recall_actions: Option<ApprovalAction>,
	#[serde(rename = "recordEditability")]
	pub record_editability: RecordEditabilityType,
	#[serde(rename = "showApprovalHistory")]
	pub show_approval_history: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
