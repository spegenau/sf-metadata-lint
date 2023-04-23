use crate::metadata::NotificationTypeSettings::NotificationTypeSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NotificationTypeConfig  {
	#[serde(rename = "notificationTypeSettings")]
	pub notification_type_settings: Option<Vec<NotificationTypeSettings>>,
}
