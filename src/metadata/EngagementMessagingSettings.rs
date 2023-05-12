use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EngagementMessagingSettings  {
	#[serde(rename = "isEngagementMessagingComplete")]
	pub is_engagement_messaging_complete: Option<bool>,
	#[serde(rename = "isEngagementMessagingEnabled")]
	pub is_engagement_messaging_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
