use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LeadConfigSettings  {
	#[serde(rename = "doesEnableLeadConvertDefaultSubjectBlankTaskCreation")]
	pub does_enable_lead_convert_default_subject_blank_task_creation: Option<bool>,
	#[serde(rename = "doesHideOpportunityInConvertLeadWindow")]
	pub does_hide_opportunity_in_convert_lead_window: Option<bool>,
	#[serde(rename = "doesPreserveLeadStatus")]
	pub does_preserve_lead_status: Option<bool>,
	#[serde(rename = "doesSelectNoOpportunityOnConvertLead")]
	pub does_select_no_opportunity_on_convert_lead: Option<bool>,
	#[serde(rename = "doesTrackHistory")]
	pub does_track_history: Option<bool>,
	#[serde(rename = "enableConversionsOnMobile")]
	pub enable_conversions_on_mobile: Option<bool>,
	#[serde(rename = "enableOrgWideMergeAndDelete")]
	pub enable_org_wide_merge_and_delete: Option<bool>,
	#[serde(rename = "shouldLeadConvertRequireValidation")]
	pub should_lead_convert_require_validation: Option<bool>,
	#[serde(rename = "shouldSendNotificationEmailWhenLeadOwnerUpdatesViaApexInLEX")]
	pub should_send_notification_email_when_lead_owner_updates_via_apex_in_lex: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
