use crate::metadata::AssignToLookupValueType::AssignToLookupValueType;
use crate::metadata::BusinessHoursSourceType::BusinessHoursSourceType;
use crate::metadata::EscalationAction::EscalationAction;
use crate::metadata::EscalationStartTimeType::EscalationStartTimeType;
use crate::metadata::FilterItem::FilterItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RuleEntry  {
	#[serde(rename = "assignedTo")]
	pub assigned_to: Option<String>,
	#[serde(rename = "assignedToType")]
	pub assigned_to_type: Option<AssignToLookupValueType>,
	#[serde(rename = "booleanFilter")]
	pub boolean_filter: Option<String>,
	#[serde(rename = "businessHours")]
	pub business_hours: Option<String>,
	#[serde(rename = "businessHoursSource")]
	pub business_hours_source: Option<BusinessHoursSourceType>,
	#[serde(rename = "criteriaItems")]
	pub criteria_items: Option<Vec<FilterItem>>,
	#[serde(rename = "disableEscalationWhenModified")]
	pub disable_escalation_when_modified: Option<bool>,
	#[serde(rename = "escalationAction")]
	pub escalation_action: Option<Vec<EscalationAction>>,
	#[serde(rename = "escalationStartTime")]
	pub escalation_start_time: Option<EscalationStartTimeType>,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "notifyCcRecipients")]
	pub notify_cc_recipients: Option<bool>,
	#[serde(rename = "overrideExistingTeams")]
	pub override_existing_teams: Option<bool>,
	#[serde(rename = "replyToEmail")]
	pub reply_to_email: Option<String>,
	#[serde(rename = "senderEmail")]
	pub sender_email: Option<String>,
	#[serde(rename = "senderName")]
	pub sender_name: Option<String>,
	#[serde(rename = "team")]
	pub team: Option<Vec<String>>,
	#[serde(rename = "template")]
	pub template: Option<String>,
}
