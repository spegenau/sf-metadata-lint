use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NotificationsSettings  {
	#[serde(rename = "enableActvityReminderBrowserNotifs")]
	pub enable_actvity_reminder_browser_notifs: Option<bool>,
	#[serde(rename = "enableMobileAppPushNotifications")]
	pub enable_mobile_app_push_notifications: Option<bool>,
	#[serde(rename = "enableNotifications")]
	pub enable_notifications: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
