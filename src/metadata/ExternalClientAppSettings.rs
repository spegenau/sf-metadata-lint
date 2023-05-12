use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalClientAppSettings  {
	#[serde(rename = "enableConsumerSecretApiAccess")]
	pub enable_consumer_secret_api_access: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
