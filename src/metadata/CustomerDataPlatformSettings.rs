use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomerDataPlatformSettings  {
	#[serde(rename = "enableCustomerDataPlatform")]
	pub enable_customer_data_platform: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
