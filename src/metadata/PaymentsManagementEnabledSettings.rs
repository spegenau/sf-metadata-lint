use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PaymentsManagementEnabledSettings  {
	#[serde(rename = "paymentsManagementEnabled")]
	pub payments_management_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
