use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SystemNotificationSettings  {
	#[serde(rename = "disableDowntimeNotifications")]
	pub disable_downtime_notifications: Option<bool>,
	#[serde(rename = "disableMaintenanceNotifications")]
	pub disable_maintenance_notifications: Option<bool>,
}
