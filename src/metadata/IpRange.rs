use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IpRange  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "end")]
	pub end: Option<String>,
	#[serde(rename = "start")]
	pub start: Option<String>,
}
