use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileApplicationVisibility  {
	#[serde(rename = "application")]
	pub application: String,
	#[serde(rename = "default")]
	pub default: bool,
	#[serde(rename = "visible")]
	pub visible: bool,
}
