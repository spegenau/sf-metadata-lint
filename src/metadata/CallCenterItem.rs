use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CallCenterItem  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "value")]
	pub value: String,
}
