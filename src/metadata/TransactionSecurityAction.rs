use crate::metadata::TransactionSecurityNotification::TransactionSecurityNotification;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TransactionSecurityAction  {
	#[serde(rename = "block")]
	pub block: bool,
	#[serde(rename = "endSession")]
	pub end_session: bool,
	#[serde(rename = "freezeUser")]
	pub freeze_user: bool,
	#[serde(rename = "notifications")]
	pub notifications: Option<Vec<TransactionSecurityNotification>>,
	#[serde(rename = "twoFactorAuthentication")]
	pub two_factor_authentication: bool,
}
