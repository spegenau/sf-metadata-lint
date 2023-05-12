use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RedirectWhitelistUrl  {
	#[serde(rename = "url")]
	pub url: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
