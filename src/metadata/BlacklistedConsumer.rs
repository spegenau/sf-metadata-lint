use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BlacklistedConsumer  {
	#[serde(rename = "blockedByApiWhitelisting")]
	pub blocked_by_api_whitelisting: bool,
	#[serde(rename = "consumerKey")]
	pub consumer_key: String,
	#[serde(rename = "consumerName")]
	pub consumer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
