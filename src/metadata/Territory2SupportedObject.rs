use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2SupportedObject  {
	#[serde(rename = "defaultAccessLevel")]
	pub default_access_level: String,
	#[serde(rename = "objectType")]
	pub object_type: String,
	#[serde(rename = "state")]
	pub state: String,
}
