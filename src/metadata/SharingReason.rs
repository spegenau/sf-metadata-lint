use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingReason  {
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
