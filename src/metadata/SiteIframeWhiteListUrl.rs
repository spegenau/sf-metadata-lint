use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SiteIframeWhiteListUrl  {
	#[serde(rename = "url")]
	pub url: String,
}
