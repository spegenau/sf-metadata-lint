use crate::metadata::ServiceChannelStatus::ServiceChannelStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ServicePresenceStatus  {
	#[serde(rename = "channels")]
	pub channels: Option<ServiceChannelStatus>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
