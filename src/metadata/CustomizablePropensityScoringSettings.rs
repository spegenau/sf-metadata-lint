use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomizablePropensityScoringSettings  {
	#[serde(rename = "enableCpsPref")]
	pub enable_cps_pref: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
