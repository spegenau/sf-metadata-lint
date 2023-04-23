use crate::metadata::AppSettings::AppSettings;
use crate::metadata::NotificationChannels::NotificationChannels;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NotificationTypeSettings  {
	#[serde(rename = "appSettings")]
	pub app_settings: Option<Vec<AppSettings>>,
	#[serde(rename = "notificationChannels")]
	pub notification_channels: Option<NotificationChannels>,
	#[serde(rename = "notificationType")]
	pub notification_type: String,
}
