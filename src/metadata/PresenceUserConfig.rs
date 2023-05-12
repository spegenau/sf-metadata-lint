use crate::metadata::PresenceConfigAssignments::PresenceConfigAssignments;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PresenceUserConfig  {
	#[serde(rename = "assignments")]
	pub assignments: Option<PresenceConfigAssignments>,
	#[serde(rename = "capacity")]
	pub capacity: i32,
	#[serde(rename = "declineReasons")]
	pub decline_reasons: Option<Vec<String>>,
	#[serde(rename = "enableAutoAccept")]
	pub enable_auto_accept: Option<bool>,
	#[serde(rename = "enableDecline")]
	pub enable_decline: Option<bool>,
	#[serde(rename = "enableDeclineReason")]
	pub enable_decline_reason: Option<bool>,
	#[serde(rename = "enableDisconnectSound")]
	pub enable_disconnect_sound: Option<bool>,
	#[serde(rename = "enableRequestSound")]
	pub enable_request_sound: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "presenceStatusOnDecline")]
	pub presence_status_on_decline: Option<String>,
	#[serde(rename = "presenceStatusOnPushTimeout")]
	pub presence_status_on_push_timeout: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
