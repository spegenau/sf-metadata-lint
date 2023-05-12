use crate::metadata::CaseSubjectOption::CaseSubjectOption;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SocialCustomerServiceSettings  {
	#[serde(rename = "caseSubjectOption")]
	pub case_subject_option: CaseSubjectOption,
	#[serde(rename = "enableAllFBResponseAccounts")]
	pub enable_all_fb_response_accounts: Option<bool>,
	#[serde(rename = "enableInboundProcessingConcurrency")]
	pub enable_inbound_processing_concurrency: Option<bool>,
	#[serde(rename = "enableSocialApprovals")]
	pub enable_social_approvals: Option<bool>,
	#[serde(rename = "enableSocialCaseAssignmentRules")]
	pub enable_social_case_assignment_rules: Option<bool>,
	#[serde(rename = "enableSocialCustomerService")]
	pub enable_social_customer_service: Option<bool>,
	#[serde(rename = "enableSocialPersonaHistoryTracking")]
	pub enable_social_persona_history_tracking: Option<bool>,
	#[serde(rename = "enableSocialPostHistoryTracking")]
	pub enable_social_post_history_tracking: Option<bool>,
	#[serde(rename = "enableSocialReceiveParentPost")]
	pub enable_social_receive_parent_post: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
