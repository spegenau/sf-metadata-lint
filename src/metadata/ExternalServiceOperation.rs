use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalServiceOperation  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "name")]
	pub name: String,
}
