use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppIpRange  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "end")]
	pub end: String,
	#[serde(rename = "start")]
	pub start: String,
}
