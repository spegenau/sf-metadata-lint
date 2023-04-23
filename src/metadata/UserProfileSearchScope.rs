use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UserProfileSearchScope  {
	#[serde(rename = "entityApiNames")]
	pub entity_api_names: Option<Vec<String>>,
	#[serde(rename = "profile")]
	pub profile: Option<String>,
}
