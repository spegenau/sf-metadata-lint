use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CorsWhitelistOrigin  {
	#[serde(rename = "urlPattern")]
	pub url_pattern: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
