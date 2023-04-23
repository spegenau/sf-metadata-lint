use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NotificationChannels  {
	#[serde(rename = "desktopEnabled")]
	pub desktop_enabled: Option<bool>,
	#[serde(rename = "mobileEnabled")]
	pub mobile_enabled: Option<bool>,
	#[serde(rename = "slackEnabled")]
	pub slack_enabled: Option<bool>,
}
