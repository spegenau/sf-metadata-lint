use crate::metadata::EmailToCaseRoutingAddressType::EmailToCaseRoutingAddressType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailToCaseRoutingAddress  {
	#[serde(rename = "addressType")]
	pub address_type: Option<EmailToCaseRoutingAddressType>,
	#[serde(rename = "authorizedSenders")]
	pub authorized_senders: Option<String>,
	#[serde(rename = "caseOrigin")]
	pub case_origin: Option<String>,
	#[serde(rename = "caseOwner")]
	pub case_owner: Option<String>,
	#[serde(rename = "caseOwnerType")]
	pub case_owner_type: Option<String>,
	#[serde(rename = "casePriority")]
	pub case_priority: Option<String>,
	#[serde(rename = "createTask")]
	pub create_task: Option<bool>,
	#[serde(rename = "emailAddress")]
	pub email_address: Option<String>,
	#[serde(rename = "emailServicesAddress")]
	pub email_services_address: Option<String>,
	#[serde(rename = "fallbackQueue")]
	pub fallback_queue: Option<String>,
	#[serde(rename = "isVerified")]
	pub is_verified: Option<bool>,
	#[serde(rename = "routingFlow")]
	pub routing_flow: Option<String>,
	#[serde(rename = "routingName")]
	pub routing_name: Option<String>,
	#[serde(rename = "saveEmailHeaders")]
	pub save_email_headers: Option<bool>,
	#[serde(rename = "taskStatus")]
	pub task_status: Option<String>,
}
