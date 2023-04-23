use crate::metadata::AssignToLookupValueType::AssignToLookupValueType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EscalationAction  {
	#[serde(rename = "assignedTo")]
	pub assigned_to: Option<String>,
	#[serde(rename = "assignedToTemplate")]
	pub assigned_to_template: Option<String>,
	#[serde(rename = "assignedToType")]
	pub assigned_to_type: Option<AssignToLookupValueType>,
	#[serde(rename = "minutesToEscalation")]
	pub minutes_to_escalation: Option<i32>,
	#[serde(rename = "notifyCaseOwner")]
	pub notify_case_owner: Option<bool>,
	#[serde(rename = "notifyEmail")]
	pub notify_email: Option<Vec<String>>,
	#[serde(rename = "notifyTo")]
	pub notify_to: Option<String>,
	#[serde(rename = "notifyToTemplate")]
	pub notify_to_template: Option<String>,
}
