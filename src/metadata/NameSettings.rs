use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct NameSettings  {
	#[serde(rename = "enableInformalName")]
	pub enable_informal_name: Option<bool>,
	#[serde(rename = "enableMiddleName")]
	pub enable_middle_name: Option<bool>,
	#[serde(rename = "enableNameSuffix")]
	pub enable_name_suffix: Option<bool>,
}
