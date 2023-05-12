use crate::metadata::IdempotencySupportStatus::IdempotencySupportStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PaymentGatewayProvider  {
	#[serde(rename = "apexAdapter")]
	pub apex_adapter: Option<String>,
	#[serde(rename = "comments")]
	pub comments: Option<String>,
	#[serde(rename = "idempotencySupported")]
	pub idempotency_supported: IdempotencySupportStatus,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
