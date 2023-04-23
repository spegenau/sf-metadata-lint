use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2AccessLevel  {
	#[serde(rename = "accessLevel")]
	pub access_level: String,
	#[serde(rename = "objectType")]
	pub object_type: String,
}
