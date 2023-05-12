use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomIndex  {
	#[serde(rename = "allowNullValues")]
	pub allow_null_values: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
