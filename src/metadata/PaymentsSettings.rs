use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PaymentsSettings  {
	#[serde(rename = "enablePayments")]
	pub enable_payments: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
