use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppAttribute  {
	#[serde(rename = "formula")]
	pub formula: String,
	#[serde(rename = "key")]
	pub key: String,
}
