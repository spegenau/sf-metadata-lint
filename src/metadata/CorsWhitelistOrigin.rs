use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CorsWhitelistOrigin  {
	#[serde(rename = "urlPattern")]
	pub url_pattern: String,
}
