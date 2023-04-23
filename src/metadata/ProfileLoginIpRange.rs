use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileLoginIpRange  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "endAddress")]
	pub end_address: String,
	#[serde(rename = "startAddress")]
	pub start_address: String,
}
