use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct State  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "integrationValue")]
	pub integration_value: String,
	#[serde(rename = "isoCode")]
	pub iso_code: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "standard")]
	pub standard: bool,
	#[serde(rename = "visible")]
	pub visible: bool,
}
