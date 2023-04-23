use crate::metadata::EmailServicesAddress::EmailServicesAddress;
use crate::metadata::EmailServicesAttOptions::EmailServicesAttOptions;
use crate::metadata::EmailServicesErrorAction::EmailServicesErrorAction;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailServicesFunction  {
	#[serde(rename = "apexClass")]
	pub apex_class: String,
	#[serde(rename = "attachmentOption")]
	pub attachment_option: EmailServicesAttOptions,
	#[serde(rename = "authenticationFailureAction")]
	pub authentication_failure_action: EmailServicesErrorAction,
	#[serde(rename = "authorizationFailureAction")]
	pub authorization_failure_action: EmailServicesErrorAction,
	#[serde(rename = "authorizedSenders")]
	pub authorized_senders: Option<String>,
	#[serde(rename = "emailServicesAddresses")]
	pub email_services_addresses: Option<Vec<EmailServicesAddress>>,
	#[serde(rename = "errorRoutingAddress")]
	pub error_routing_address: Option<String>,
	#[serde(rename = "functionInactiveAction")]
	pub function_inactive_action: EmailServicesErrorAction,
	#[serde(rename = "functionName")]
	pub function_name: String,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isAuthenticationRequired")]
	pub is_authentication_required: Option<bool>,
	#[serde(rename = "isErrorRoutingEnabled")]
	pub is_error_routing_enabled: Option<bool>,
	#[serde(rename = "isTextAttachmentsAsBinary")]
	pub is_text_attachments_as_binary: Option<bool>,
	#[serde(rename = "isTlsRequired")]
	pub is_tls_required: Option<bool>,
	#[serde(rename = "overLimitAction")]
	pub over_limit_action: EmailServicesErrorAction,
}
