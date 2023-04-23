use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileSearchLayouts  {
	#[serde(rename = "fields")]
	pub fields: Option<Vec<String>>,
	#[serde(rename = "profileName")]
	pub profile_name: Option<String>,
}
