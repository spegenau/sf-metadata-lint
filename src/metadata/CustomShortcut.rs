use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomShortcut  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "eventName")]
	pub event_name: String,
}
