use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailServicesAddress  {
	#[serde(rename = "authorizedSenders")]
	pub authorized_senders: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "localPart")]
	pub local_part: String,
	#[serde(rename = "runAsUser")]
	pub run_as_user: String,
}
