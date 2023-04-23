use crate::metadata::State::State;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Country  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "integrationValue")]
	pub integration_value: String,
	#[serde(rename = "isoCode")]
	pub iso_code: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "orgDefault")]
	pub org_default: bool,
	#[serde(rename = "standard")]
	pub standard: bool,
	#[serde(rename = "states")]
	pub states: Option<Vec<State>>,
	#[serde(rename = "visible")]
	pub visible: bool,
}
