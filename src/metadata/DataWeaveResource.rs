use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DataWeaveResource  {
	#[serde(rename = "apiVersion")]
	pub api_version: f32,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
}
