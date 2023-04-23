use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValueSettings  {
	#[serde(rename = "controllingFieldValue")]
	pub controlling_field_value: Option<Vec<String>>,
	#[serde(rename = "valueName")]
	pub value_name: String,
}
