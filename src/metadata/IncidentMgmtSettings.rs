use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IncidentMgmtSettings  {
	#[serde(rename = "enableAlertBroadcastType")]
	pub enable_alert_broadcast_type: Option<bool>,
	#[serde(rename = "enableEmailBroadcastType")]
	pub enable_email_broadcast_type: Option<bool>,
	#[serde(rename = "enableIncidentMgmt")]
	pub enable_incident_mgmt: Option<bool>,
	#[serde(rename = "enableSiteBannerBroadcastType")]
	pub enable_site_banner_broadcast_type: Option<bool>,
	#[serde(rename = "enableSlackBroadcastType")]
	pub enable_slack_broadcast_type: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
