use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AccessMapping  {
	#[serde(rename = "accessLevel")]
	pub access_level: String,
	#[serde(rename = "object")]
	pub object: String,
	#[serde(rename = "objectField")]
	pub object_field: String,
	#[serde(rename = "userField")]
	pub user_field: String,
}
