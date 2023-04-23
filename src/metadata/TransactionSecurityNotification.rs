use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TransactionSecurityNotification  {
	#[serde(rename = "inApp")]
	pub in_app: bool,
	#[serde(rename = "sendEmail")]
	pub send_email: bool,
	#[serde(rename = "user")]
	pub user: String,
}
