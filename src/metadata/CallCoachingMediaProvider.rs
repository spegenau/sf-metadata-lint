use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CallCoachingMediaProvider  {
	#[serde(rename = "isActive")]
	pub is_active: bool,
	#[serde(rename = "providerDescription")]
	pub provider_description: String,
	#[serde(rename = "providerName")]
	pub provider_name: String,
}
