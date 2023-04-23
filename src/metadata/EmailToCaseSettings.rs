use crate::metadata::EmailToCaseOnFailureActionType::EmailToCaseOnFailureActionType;
use crate::metadata::EmailToCaseRoutingAddress::EmailToCaseRoutingAddress;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailToCaseSettings  {
	#[serde(rename = "enableE2CAttachmentAsFile")]
	pub enable_e_2_c_attachment_as_file: Option<bool>,
	#[serde(rename = "enableE2CSourceTracking")]
	pub enable_e_2_c_source_tracking: Option<bool>,
	#[serde(rename = "enableEmailToCase")]
	pub enable_email_to_case: Option<bool>,
	#[serde(rename = "enableHtmlEmail")]
	pub enable_html_email: Option<bool>,
	#[serde(rename = "enableOnDemandEmailToCase")]
	pub enable_on_demand_email_to_case: Option<bool>,
	#[serde(rename = "enableThreadIDInBody")]
	pub enable_thread_id_in_body: Option<bool>,
	#[serde(rename = "enableThreadIDInSubject")]
	pub enable_thread_id_in_subject: Option<bool>,
	#[serde(rename = "notifyOwnerOnNewCaseEmail")]
	pub notify_owner_on_new_case_email: Option<bool>,
	#[serde(rename = "overEmailLimitAction")]
	pub over_email_limit_action: Option<EmailToCaseOnFailureActionType>,
	#[serde(rename = "preQuoteSignature")]
	pub pre_quote_signature: Option<bool>,
	#[serde(rename = "routingAddresses")]
	pub routing_addresses: Option<Vec<EmailToCaseRoutingAddress>>,
	#[serde(rename = "unauthorizedSenderAction")]
	pub unauthorized_sender_action: Option<EmailToCaseOnFailureActionType>,
}
