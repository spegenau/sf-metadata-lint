use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ListPlacement  {
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "location")]
	pub location: String,
	#[serde(rename = "units")]
	pub units: Option<String>,
	#[serde(rename = "width")]
	pub width: Option<i32>,
}
