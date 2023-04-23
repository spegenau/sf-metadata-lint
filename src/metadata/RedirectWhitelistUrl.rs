use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RedirectWhitelistUrl  {
	#[serde(rename = "url")]
	pub url: String,
}
