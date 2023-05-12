use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommerceSettings  {
	#[serde(rename = "commerceEnabled")]
	pub commerce_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
