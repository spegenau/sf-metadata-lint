use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PlatformSlackSettings  {
	#[serde(rename = "enableSlackService")]
	pub enable_slack_service: Option<bool>,
	#[serde(rename = "enableSlackServiceAlerts")]
	pub enable_slack_service_alerts: Option<bool>,
	#[serde(rename = "slackCapabilitiesEnabled")]
	pub slack_capabilities_enabled: Option<bool>,
}
