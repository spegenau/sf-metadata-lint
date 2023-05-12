use crate::metadata::PlatformEventChannelType::PlatformEventChannelType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformEventChannel  {
	#[serde(rename = "channelType")]
	pub channel_type: PlatformEventChannelType,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
