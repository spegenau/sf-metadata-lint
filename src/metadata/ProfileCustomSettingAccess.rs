use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileCustomSettingAccess  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "name")]
	pub name: String,
}
