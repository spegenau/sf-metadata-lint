use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppMenuItem  {
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "type")]
	pub _type: String,
}
