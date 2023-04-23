use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileApexPageAccess  {
	#[serde(rename = "apexPage")]
	pub apex_page: String,
	#[serde(rename = "enabled")]
	pub enabled: bool,
}
