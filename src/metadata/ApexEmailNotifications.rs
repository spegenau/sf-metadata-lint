use crate::metadata::ApexEmailNotification::ApexEmailNotification;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApexEmailNotifications  {
	#[serde(rename = "apexEmailNotification")]
	pub apex_email_notification: Option<Vec<ApexEmailNotification>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
