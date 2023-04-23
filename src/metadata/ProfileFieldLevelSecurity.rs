use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileFieldLevelSecurity  {
	#[serde(rename = "editable")]
	pub editable: bool,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "readable")]
	pub readable: Option<bool>,
}
